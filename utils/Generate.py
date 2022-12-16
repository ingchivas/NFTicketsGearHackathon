# -*- coding: utf-8 -*-
"""
Created on Wed Dec 14 18:50:23 2022

@author: Claudia
"""

from Generator import generator


directory = "./flk"
testWallet = "126dp8jqShqW65FvNkcb9cTMd1kydPMhmbmohuwmjY347qVN"

def generate(n : int = 1 , wallet : str = testWallet):
    generator_object = generator(directory,wallet)
    generator_object.generate_multiple_nfts(n)
    generator_object.write_combinations()

if __name__ == '__main__':
    generate(10,testWallet)
    print("succesful creation")