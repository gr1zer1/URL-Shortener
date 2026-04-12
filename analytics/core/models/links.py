from .base import Base
from sqlalchemy.orm import Mapped,mapped_column
from sqlalchemy import String,Integer

class LinkModel(Base):
    code:Mapped[str] = mapped_column(String,unique=True,index=True)
    clicks:Mapped[int] = mapped_column(Integer)
    last_click:Mapped[str] = mapped_column(String)