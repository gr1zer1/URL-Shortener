import redis.asyncio as redis
import json
from .schemas import LinkSchema
from pydantic import ValidationError
from core.models import LinkModel
from sqlalchemy import update
from core.db import db_helper

async def create_connection():
    conn = redis.Redis(host="redis", port=6379, decode_responses=True)
    async with db_helper.session_factory() as session:
        while True:
            queue, data = await conn.brpop(["analytics:clicks"], timeout=0)
            try:
                raw = json.loads(data)
                res = LinkSchema.model_validate(raw)
                stmt = (
                    update(LinkModel)
                    .where(LinkModel.code == res.code)
                    .values(clicks=LinkModel.clicks + 1, last_click=res.last_click)
                )
                result = await session.execute(stmt)
                if result.rowcount == 0:
                    link = LinkModel(code=res.code, clicks=1, last_click=res.last_click)
                    session.add(link)
                await session.commit()
            except (json.JSONDecodeError, ValidationError) as e:
                print("BAD EVENT:", e)
                await session.rollback()