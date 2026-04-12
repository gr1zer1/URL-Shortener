from pydantic import BaseModel, ConfigDict


class LinkSchema(BaseModel):
    code: str
    timestamp: str


class LinkResponseSchema(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    code: str
    clicks: int
    last_click: str
