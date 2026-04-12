from sqlalchemy.orm import DeclarativeBase,declared_attr,Mapped,mapped_column


class Base(DeclarativeBase):
    
    @declared_attr.directive
    def __tablename__(cls) -> str:
        return f"{cls.__name__[:-5].lower()}s"
    
    id:Mapped[int] = mapped_column(primary_key=True)