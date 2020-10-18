#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Fri Sep 25 21:14:09 2020

@author: john
"""


import os
import struct

from plot_signal_summary import plot

filename = 'output_25.00MHz_1.0Msps_gain40.0dB_sc8.dat'

N_SAMPLES = 1e6
N_AVG_DOWNSAMPLE = 1

_, tail = os.path.split(filename)
tail_split = tail.split('_')
center_freq = float(tail_split[1][:-3])*1.0e6
rate_sps = float(tail_split[2][:-4])*(1.0e6 / N_AVG_DOWNSAMPLE)

f_in = open(filename, 'rb')

assert(tail_split[4] == 'sc8.dat')

data = []
downsample_accum = []
buffer = f_in.read(2)
while len(buffer) == 2 and len(data) < N_SAMPLES:
    re, im = struct.unpack('<bb', buffer)
    downsample_accum.append(re + 1j*im)
    if len(downsample_accum) >= N_AVG_DOWNSAMPLE:
        data.append(sum(downsample_accum) / N_AVG_DOWNSAMPLE)
        downsample_accum = []
    buffer = f_in.read(2)
    
plot(data, tail, center_freq, rate_sps)    

f_in.close()

