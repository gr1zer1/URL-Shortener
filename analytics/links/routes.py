from typing import Annotated, List

from core.db import db_helper
from core.models import LinkModel
from fastapi import Depends
from fastapi.routing import APIRouter
from sqlalchemy import select
from sqlalchemy.ext.asyncio import AsyncSession

from .schemas import LinkResponseSchema

SessionDep = Annotated[AsyncSession, Depends(db_helper.get_session)]


app = APIRouter()


@app.get("/links")
async def get_links(session: SessionDep) -> List[LinkResponseSchema]:
    stmt = select(LinkModel).order_by(LinkModel.clicks)
    res = await session.execute(stmt)
    data = res.scalars().all()

    return data
