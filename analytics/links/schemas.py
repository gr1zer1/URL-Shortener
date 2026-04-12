from pydantic import BaseModel

class LinkSchema(BaseModel):
    code:str
    clicks:int
    timestamp:str

class LinkResponseSchema(BaseModel):
    code:str
    clicks:int
    last_click:str
