import requests
import json
import os
import re

session = os.getenv("AOC_SESSION")
headers = {"Cookie": f"session={session}"}

years = range(2015, 2025)

for year in years:
    url = f"https://adventofcode.com/{year}/leaderboard/self"
    res = requests.get(url, headers=headers)

    if res.status_code != 200:
        stars = "0"
    else:
        match = re.search(r"(\d+)\s+stars", res.text)
        stars = match.group(1) if match else "0"

    data = {
        "label": f"AoC {year}",
        "message": stars,
        "color": "yellow"
    }

    with open(f"aoc_{year}.json", "w") as f:
        json.dump(data, f)
