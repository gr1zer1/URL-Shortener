from fastapi.routing import APIRouter
from fastapi import Depends

from .schemas import LinkResponseSchema
from core.models import LinkModel
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select
from typing import Annotated
from core.db import db_helper



SessionDep = Annotated[AsyncSession,Depends(db_helper.get_session)]


app = APIRouter()

@app.get("/links")
async def get_links(session:SessionDep) -> LinkResponseSchema:
    stmt = select(LinkModel).order_by(LinkModel.clicks)
    res = await session.execute(stmt)
    data = res.scalars().all()

    return data


    


