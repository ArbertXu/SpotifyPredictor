import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt


df = pd.read_csv("features.csv")


df["popular"] = df["popular"].map({0: "Not Popular", 1: "Popular"})


features = df.columns[:-1]

for feature in features:
    plt.figure(figsize=(8, 4))
    sns.kdeplot(data=df, x=feature, hue="popular", common_norm=False, fill=True)
    plt.title(f"Distribution of {feature}")
    plt.show()
