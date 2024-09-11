#     """Example of how to use the SurrealDB client."""
#     async with Surreal("ws://localhost:8000/rpc") as db:
# Run Rust code to generate the token and paste in here ->

token="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJpYXQiOjE3MjYwNzY3MDUsIm5iZiI6MTcyNjA3NjcwNSwiZXhwIjoxNzI2MDgwMzA1LCJpc3MiOiJTdXJyZWFsREIiLCJqdGkiOiI4OGFhOWQ0Ni1hZmE0LTRiYjgtODhmNC1mMDUzNDMzYTllMTkiLCJOUyI6InRlc3QiLCJEQiI6InRlc3QiLCJTQyI6ImFkbWluIiwiSUQiOiJ1c2VyOnp0c2FrcDIzZnl4Z21hcGJvODJtIn0.q3iNFVir871EldBJSTy2gzi8XsjKBvCM0gPE3X9pmXobCjI220-hW25960mKGEeXi_eXPaEECHe8NXw0HgaaWg"

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
