import scipy.io
import csv


def convert(source: str, target: str):
    df = scipy.io.loadmat(source)["DATA_ORIGINAL"]
    with open(target, 'w', newline='') as csvfile:
        fieldnames = ['ch1', 'ch2']
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)

        writer.writeheader()
        for i in range(3202):
            writer.writerow({'ch1': df[0][i], 'ch2': df[1][i]})


if __name__ == '__main__':
    for prefix in ["donnees_", "donnees_vide_"]:
        for i in range(1, 17):
            n = i - 1
            convert(f"./data/{prefix}{i:02}.mat", f"./data/csv/{prefix}{n}.csv")
