#!/usr/bin/env python3
import asyncio
import json
import random
import string
import websockets

rooms = {}

def generate_code():
    return ''.join(random.choices(string.digits, k=6))

async def handler(websocket):
    room_code = None
    try:
        async for message in websocket:
            data = json.loads(message)
            msg_type = data.get("type")

            if msg_type == "create":
                code = generate_code()
                while code in rooms:
                    code = generate_code()
                rooms[code] = {"players": [websocket], "host": websocket}
                room_code = code
                await websocket.send(json.dumps({"type": "created", "code": code}))

            elif msg_type == "join":
                code = data.get("code", "")
                if code not in rooms:
                    await websocket.send(json.dumps({"type": "error", "message": "房間不存在"}))
                elif len(rooms[code]["players"]) >= 2:
                    await websocket.send(json.dumps({"type": "error", "message": "房間已滿"}))
                else:
                    rooms[code]["players"].append(websocket)
                    room_code = code
                    for p in rooms[code]["players"]:
                        role = "host" if p == rooms[code]["host"] else "guest"
                        await p.send(json.dumps({"type": "start", "role": role, "code": code}))

            elif msg_type == "state":
                if room_code and room_code in rooms:
                    players = rooms[room_code]["players"]
                    opponent = players[0] if players[1] == websocket else players[1]
                    try:
                        await opponent.send(json.dumps({
                            "type": "opponent_state",
                            "board": data.get("board"),
                            "score": data.get("score", 0),
                            "level": data.get("level", 1),
                            "lines": data.get("lines", 0)
                        }))
                    except websockets.exceptions.ConnectionClosed:
                        pass

            elif msg_type == "gameover":
                if room_code and room_code in rooms:
                    players = rooms[room_code]["players"]
                    opponent = players[0] if players[1] == websocket else players[1]
                    try:
                        await opponent.send(json.dumps({
                            "type": "opponent_gameover",
                            "score": data.get("score", 0),
                            "level": data.get("level", 1),
                            "lines": data.get("lines", 0)
                        }))
                    except websockets.exceptions.ConnectionClosed:
                        pass

    except websockets.exceptions.ConnectionClosed:
        pass
    finally:
        if room_code and room_code in rooms:
            players = rooms[room_code]["players"]
            for p in players:
                if p != websocket:
                    try:
                        await p.send(json.dumps({"type": "opponent_disconnected"}))
                    except websockets.exceptions.ConnectionClosed:
                        pass
            del rooms[room_code]

async def main():
    print("WebSocket server running on ws://0.0.0.0:3000")
    async with websockets.serve(handler, "0.0.0.0", 3000):
        await asyncio.Future()

if __name__ == "__main__":
    asyncio.run(main())