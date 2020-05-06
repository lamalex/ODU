# Read Me!
----------

## Obtaining data
The script ```get_all_data.sh``` has been provided to aid the user in downloading requisite data files.

```bash
~$ ./get_all_data.sh
```
This will download
* Citibike trip data
* 2013-2016 NYC tax data
* Zipcode GIS polygons and associated metadata

**Note**: ```get_all_data.sh``` will ```wget``` a large amount of data. The Citibike data alone is over 6GB.
A minimal set of data has been distributed with this project for easy testing.

## The easy way
2 slurm scripts have been provided for use on Turing. Simply execute:
```bash
~$ sbatch citibike.slurm
# and upon its completion (about 5 mins)
~$ sbatch visualize.slurm ziptrips.<timestamp>.pickle
```

## The hard(er) way
### Additional python dependencies needed
This application relies on some additional python modules outside of a standard python installation.
* pandas
* geopandas
* matplotlib
* dateparser
* shapely

They can easily be installed with ```pip```.
```bash
~$ pip install pandas geopandas matplotlib dateparser shapely
```
### Processing the data
```bash
~$ spark-submit code/main.py
```
Which outputs a python pickle named ```ziptrips.<timestamp>.pkl```.

```bash
~$ python code/visualize.py <path-to-pickle>
```
```visualize.py``` will generate a series of graphs plotting number of trips over time for a zip code, and changed in income tax return over the same time period.
