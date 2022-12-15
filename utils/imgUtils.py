# Function that creates a hex color palette from a crypto wallet address

import hashlib
import binascii
import random
import os

def wallet2pallete(walletAddr):
    # Convert the wallet address to a hex string
    walletAddr = walletAddr.encode('utf-8')
    walletAddr = binascii.hexlify(walletAddr)
    walletAddr = walletAddr.decode('utf-8')

    # Hash the wallet address
    walletAddr = hashlib.sha256(walletAddr.encode('utf-8')).hexdigest()

    # Create a list of hex colors from the hash
    hexColors = []
    for i in range(0, len(walletAddr), 6):
        hexColors.append('#' + walletAddr[i:i+6])

    # Return the list of hex colors
    return hexColors

import matplotlib.pyplot as plt

def plotColors(colors):
    # Create a plot of the colors
    fig, ax = plt.subplots()
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.set_axis_off()
    for i, color in enumerate(colors):
        ax.add_patch(plt.Rectangle((0, i/len(colors)), 1, 1/len(colors), color=color))
    plt.show()


testWallet = "126dp8jqShqW65FvNkcb9cTMd1kydPMhmbmohuwmjY347qVN"

colors = wallet2pallete(testWallet)

plotColors(colors)
