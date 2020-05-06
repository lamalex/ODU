#!/bin/bash -l

#SBATCH --ntasks=1
#SBATCH --mail-type=ALL
#SBATCH --mail-user=alaun001@odu.edu

enable_lmod
module load python
module load pandas
module load matplotlib

mkdir -p results
python code/visualize.py $1
