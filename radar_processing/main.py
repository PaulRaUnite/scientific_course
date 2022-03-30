# 1. Average 16 measurements
# 2. compute FFT (fast furier transform) of the average
# 3.
# 4. look at
#    the data
#    the background data
#    the subtraction
import scipy.io
import numpy as np
import matplotlib.pyplot as plot


def proc(prefix: str) -> (np.ndarray, np.ndarray):
    data = np.zeros((16, 3202), dtype=int)
    for i in range(0, 16):
        n = i + 1
        data[i] = scipy.io.loadmat(f"./data/{prefix}{n:02}.mat")["DATA_ORIGINAL"][1]
        data[i, 0:20] = np.repeat(-32768, 20) # clean the start data (changes nothing)
    averaged = np.average(data, axis=0)
    averaged = np.fft.fft(averaged)
    fft = np.absolute(averaged)[:1601]

    return 20 * np.log10((2 / 50) * fft)


if __name__ == '__main__':
    with_object_frequencies = proc("donnees_")
    background_frequencies = proc("donnees_vide_")

    speed_of_light = 299792458  # [m / s]
    slope = 1950037684072.203400  # [Hz / s]
    Fs = 6250000
    half = with_object_frequencies.shape[0]
    frequency = np.linspace(0, (Fs // 2), half)
    distance = (speed_of_light * frequency) / (2 * slope)

    plot.plot(distance, with_object_frequencies)
    plot.show(figsize=(12, 8))

    plot.plot(distance, background_frequencies)
    plot.show()

    without_background = with_object_frequencies - background_frequencies
    plot.plot(distance[0:100], without_background[0:100])
    plot.show(size=(30, 8))
