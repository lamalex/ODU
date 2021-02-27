#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Thu Apr 25 21:43:56 2019

@author: alexlauni
"""
import os
import pickle
import pandas as pd
import matplotlib.pyplot as plt

f = open('code/ziptrips.pkl', 'rb')
trips = pickle.load(f)

returns = pd.DataFrame()
irs_path = os.path.join('data', 'irs_tax_data')
for file in os.listdir(irs_path):
    if not file.endswith("xls"):
        continue
    fpath = os.path.join(irs_path, file)
    df = pd.read_excel(fpath)
    df.rename(index = str, inplace = True,
        columns={"NEW YORK": "ZIP CODE", "Unnamed: 1": "TAX BRACKET",
        "Unnamed: 2": "NUM RETURNS"})
    df.reset_index(inplace=True)
    df = df.loc[:, 'ZIP CODE':'NUM RETURNS']
    df = df[14:]
    df.dropna(axis = 'index', how = 'any', inplace = True)
    pivot = df.pivot(index = 'ZIP CODE', columns = 'TAX BRACKET', values = 'NUM RETURNS')
    pivot.reset_index(inplace = True)
    pivot.index = [pd.to_datetime(str("20" + file[:2] + "01"), format="%Y%m")] * len(pivot)
    returns = returns.append(pivot, sort=True)

tframe = pd.DataFrame(data = trips)
tframe.index = pd.to_datetime(tframe.index, format="%Y%m")
tframe.sort_index(inplace=True)

def generate_graph(zipcode):
    try:
        zipcode_s = str(zipcode)

        tax_for_zip_data = returns[returns['ZIP CODE'] == zipcode]
        tax_for_zip_data = tax_for_zip_data.drop(columns=['ZIP CODE'])
        tax_for_zip_data.sort_index(inplace=True)

        fig, ax1 = plt.subplots(sharex = True)
        color = "k" #black

        ax1.set_xlabel('Date')
        ax1.set_ylabel('Trips', color = color)
        tframe.plot(kind = 'line', y = zipcode_s, ax = ax1, color = color)
        ax1.tick_params(axis = 'y', labelcolor = color)

        ax2 = ax1.twinx()
        ax2.set_ylabel('Num Returns', color = color)
        tax_for_zip_data.plot(kind = 'line', y = tax_for_zip_data.columns, ax = ax2)
        ax2.tick_params(axis = 'y', labelcolor = color)

        plt.tight_layout()
        plt.legend(loc=9, bbox_to_anchor=(0.5, -0.1), ncol=2)

        zip_graph = os.path.join('results', zipcode_s)
        plt.savefig(zip_graph, additional_artists=[plt.legend(loc=9, bbox_to_anchor=(0.5, -0.1), ncol=2)],bbox_inches="tight")
    except KeyError:
        # Not every zip code in New York has a Citibike station, so we raise a
        # KeyError when we try to generate_graph on those zip codes.
        return
    finally:
        plt.close()
        return None

bh = returns['ZIP CODE'].apply(generate_graph)
