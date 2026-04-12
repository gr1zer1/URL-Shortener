import asyncio
from contextlib import asynccontextmanager

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from links.routes import app as link_router
from links.utility import create_connection


@asynccontextmanager
async def lifespan(app: FastAPI):
    task = asyncio.create_task(create_connection())
    yield
    task.cancel()


app = FastAPI(lifespan=lifespan)
app.include_router(link_router)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8081)
