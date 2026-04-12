import json

import redis.asyncio as redis
from core.db import db_helper
from core.config import config
from core.models import LinkModel
from pydantic import ValidationError
from sqlalchemy import update


from .schemas import LinkSchema


async def create_connection():
    conn = redis.from_url(config.redis_url, decode_responses=True)
    async with db_helper.session_factory() as session:
        while True:
            queue, data = await conn.brpop(["analytics:clicks"], timeout=0)
            try:
                raw = json.loads(data)
                res = LinkSchema.model_validate(raw)
                stmt = (
                    update(LinkModel)
                    .where(LinkModel.code == res.code)
                    .values(clicks=LinkModel.clicks + 1, last_click=res.timestamp)
                )
                result = await session.execute(stmt)
                if result.rowcount == 0:
                    link = LinkModel(code=res.code, clicks=1, last_click=res.timestamp)
                    session.add(link)
                await session.commit()
            except (json.JSONDecodeError, ValidationError) as e:
                print("BAD EVENT:", e)
                await session.rollback()
