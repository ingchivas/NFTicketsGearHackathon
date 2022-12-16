# -*- coding: utf-8 -*-
"""
Created on Wed Dec 14 18:52:18 2022

@author: Claudia
"""

import os
import itertools 
from PIL import Image , ImageColor
import cv2 as cv
import random
import ast
import wallet


class generator: 
    def __init__(self, images_path: str , wallet : str ):
        #./images 
        self.images = images_path
        self.combinations_len = 0
        self.wallet =  wallet
        # Output directory 
        self.output_path: str = "./output"+ f"{self.images}"[2:]
        self.text_combinations_path:str = "./combinations" + f"{self.images}.txt"[2:]
        os.makedirs(self.output_path, exist_ok=True)
    
    def load_images_paths(self, images:str ):
        sub_paths = sorted(os.listdir(images))
        layers = []
        
        for count , single_path in enumerate(sub_paths):
            layer_path =  os.path.join(images, single_path)
            multiple_images = os.listdir(layer_path)
            layer_images= []
            
            for single_image in multiple_images:
                image_path = os.path.join(images,single_path,single_image)
                layer_images.append(f"{image_path}")
            random.shuffle(layer_images)
            layers.append(layer_images)

        return layers 
    
    def create_combinations(self):
        #cartesian_product 
        self.unique = list(itertools.product(*self.layers))
        random.shuffle(self.unique)
    
    def read_images_from_dir(self):
        # ./images directory is used to get all images inside 
        self.layers = self.load_images_paths(self.images)
        
        #creates every posible combinations with the distinct images 
        self.create_combinations()  
        

    def get_size(self):
        temporary_image = cv.imread(self.unique[0][0])
        self.max_h = temporary_image.shape[0]
        self.max_w = temporary_image.shape[1]
    
        
        
    def save_image(self, image: Image.Image, i: int = 1):
        image_index = str(i).zfill(6)
        image_file_name = f"NFT_{image_index}.png"
        image_save_path = os.path.join(self.output_path, image_file_name)
        image.save(image_save_path)
    
    def get_combination(self):
        combination = self.unique[-1]
        self.unique.pop()
        return combination
         
        
    def render_image(self):
        image = Image.new('RGBA' ,(self.max_w,self.max_h),  self.get_RGB_color())
        image_path_sequence = self.get_combination()
        for image_path in image_path_sequence:
            layer_image = Image.open(image_path)
            image = Image.alpha_composite(image, layer_image)
        return image 
    
    def get_RGB_color(self):
        pallete = wallet.wallet2pallete(self.wallet)
        palette_rgb = []
        for i in pallete :
            palette_rgb.append(ImageColor.getcolor(i,"RGB"))
        
        return palette_rgb[random.randint(0, 10)]
    
    def generate_combinations(self, n: int = 1):
        self.read_images_from_dir() #get all combinations possible
        #self.unique contains every combination of the images  
        self.combinations_len = len(self.unique)
        
    def write_combinations(self, ):
        file = open(self.text_combinations_path, mode= 'w')
        self.combinations_len = len(self.unique)
        file.write(f"{self.unique}\n{self.combinations_len}")
        file.close()
    def read_combinations(self):
        file = open(self.text_combinations_path , mode= 'r')
        self.unique = ast.literal_eval(file.readline())
        self.combinations_len = int(file.readline())
        print(len(self.unique))
        print(type(self.unique))
        
        file.close()

    def generate_multiple_nfts(self, n: int  = 1):
        if os.path.exists(self.text_combinations_path):
            print(0)        
        else:
            file = open(self.text_combinations_path, mode = 'w')
            file.close()
            print(1)
        if os.stat(self.text_combinations_path).st_size == 0:
            self.generate_combinations()
        else:   
            self.read_combinations()
        
        
        self.get_size()
        for i in range(n):
            image = self.render_image()
            self.save_image(image, self.combinations_len)
            print(self.combinations_len)
            self.combinations_len-= 1;
    
        