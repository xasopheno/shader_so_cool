import os
from pathlib import Path

datadir = Path(".")
for file in datadir.glob('*.csv'):
    short = file.name.split("_chanel_")[1]
    short = short.split("_array_")[0]
    short = "_".join(short.split(" ")).lower()
    short = short + ".csv"
    print(short)
    os.rename(file.name, short)
