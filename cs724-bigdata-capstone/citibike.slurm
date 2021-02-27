#!/bin/bash -l

#SBATCH --ntasks=1
#SBATCH --mail-type=ALL
#SBATCH --mail-user=alaun001@odu.edu

enable_lmod
module load python
module load spark

pip install --user geopandas dateparser
spark-submit code/main.py
