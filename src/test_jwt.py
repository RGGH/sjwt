#     """Example of how to use the SurrealDB client."""
#     async with Surreal("ws://localhost:8000/rpc") as db:
# Run Rust code to generate the token and paste in here ->


token="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJpYXQiOjE3MjYwNjM1ODMsIm5iZiI6MTcyNjA2MzU4MywiZXhwIjoxNzI2MDY3MTgzLCJpc3MiOiJTdXJyZWFsREIiLCJqdGkiOiJkYzJmODM3Zi1kZjYwLTQzMTAtOTY1OS0wMzFlZmZkZmY4ZDYiLCJOUyI6InRlc3QiLCJEQiI6InRlc3QiLCJTQyI6ImFkbWluIiwiSUQiOiJ1c2VyOmp4aWdvNTRycWRibGUzc2R0eHFvIn0.QW85GLXk9zXYRjU0CvMTIgNLSevtcEfsv2wYnSGwP54pgu6a13ncV1At7iF9yGgW4MCFJPztwjIdAiVF3MghOw"

from surrealdb import Surreal
import asyncio
    
async def main():
    async with Surreal("ws://localhost:8000/rpc") as db:
        print("Attempting to authenticate...")
        await db.authenticate(token)
        print("Authentication successful.")
        await db.signin({"user": "root", "pass": "root"})
        await db.use("test_namespace", "test_database")

    
        res = await db.create(
            "character",
            {
                "name": "Elvis Presley",
                "race": "human",
            },
        )
        print(res)
    
        res = await db.create(
            "character",
            {
                "name": "Dennis the Menace",
                "race": "human",
            },
        )
        print(res)
    
        res = await db.create(
            "character",
            {
                "name": "Boss Hogg",
                "race": "Sheriff",
            },
        )
        print(res)
    
        res = await db.select("character")
        print(res)
    
        res = await db.query("SELECT * FROM character")
        print(res)
    
        res = await db.query("SELECT * FROM character WHERE race='human'")
        print(res)
    
        res = await db.query("SELECT * FROM character WHERE race=$race", {
            'race': "Sheriff",
        })
        print(res)
    
        #res = await db.update("character", {
        #    "race": "na'vi",
        #})
    
        res = await db.query("UPDATE character SET race=$race WHERE name=$name", {
            'race': "na'vi",
            'name': "Jake Sully"
        })
        print(res)
    
        res = await db.select("character")
        print(res)
    
        res = await db.delete("character")
        print(res)
    
    
asyncio.run(main())
