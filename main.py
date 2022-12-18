import ctypes

engine = ctypes.WinDLL("engine/target/debug/engine.dll")

print(engine.analyze_position(3, """
    {
        "d": {
            "3": 5,
            "7": 2
        }
    }
""".encode('utf-8')))