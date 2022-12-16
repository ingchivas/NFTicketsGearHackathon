# -*- coding: utf-8 -*-
"""
Created on Wed Dec 14 18:50:23 2022

@author: Claudia
"""

from Generator import generator


directory = ".\images"

def generate(n : int = 1):
    generator_object = generator(directory)
    generator_object.generate_multiple_nfts(n)
    generator_object.write_combinations()

if __name__ == '__main__':
    generate(15)
    print("succesful creation")