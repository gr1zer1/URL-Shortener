from fastapi import FastAPI
from links.routes import app as link_router
from links.utility import create_connection

from contextlib import asynccontextmanager

import asyncio

@asynccontextmanager
async def lifespan(app: FastAPI):
    task = asyncio.create_task(create_connection())  
    task.cancel()

app = FastAPI()
app.include_router(link_router)
