import os
import sys
import matplotlib.pyplot as plt
import json

if __name__ == "__main__":
    dir = sys.argv[1]
    spike_data = json.load(dir + "/spike_data.json")
    for syn in spike_data:
        print(syn)
        #plt.plot()
    