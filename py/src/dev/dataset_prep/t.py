import pandas as pd
import numpy as np

d1 = pd.read_csv("f1.csv", index_col="ts")
d1.index = pd.to_datetime(d1.index)
d1.rename(columns={"feature": "f1"}, inplace=True)

d2 = pd.read_csv("f2.csv", index_col="ts")
d2.index = pd.to_datetime(d2.index)
d2.rename(columns={"feature": "f2"}, inplace=True)

d3 = pd.read_csv("f4.csv", index_col="ts")
d3.index = pd.to_datetime(d3.index)
d3.rename(columns={"feature": "f4"}, inplace=True)

mp = pd.read_csv("mp.csv", index_col="ts")
mp.index = pd.to_datetime(mp.index)

d4 = mp.join(d1).join(d2).join(d3)

d4["timestamp"] = d4.index.astype(np.int64)

d4.to_csv("dataset.csv", na_rep="nan", date_format="%Y-%m-%dT%H:%M:%S.%fZ")
d4.to_records(index=False).tofile("dataset.bin")
