from pathlib import Path
from pydantic import Field
from pydantic_settings import BaseSettings, SettingsConfigDict


class Config(BaseSettings):
    database_url_analytics: str = Field(validation_alias="DATABASE_URL_ANALYTICS")
    redis_url: str = Field(validation_alias="REDIS_URL")

    model_config = SettingsConfigDict(
        env_file=str(Path(__file__).parent.parent.parent / ".env"),
        extra="ignore"
    )


config = Config()