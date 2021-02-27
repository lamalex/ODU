#!/bin/sh

# Get all citibike data
DATADIR='data/citibike'
mkdir -p $DATADIR
cd $DATADIR
for j in `seq 2013 2016`
do
	for i in `seq -w 12`
	do
		wget https://s3.amazonaws.com/tripdata/${j}${i}-citibike-tripdata.zip -O ${j}${i}.zip
		unzip ${j}${i}.zip
	done
done
rm *.zip

cd ../..

# Get all tax data
DATADIR='data/irs_tax_data'
mkdir -p $DATADIR
cd $DATADIR
for j in `seq 13 16`
do
	wget https://www.irs.gov/pub/irs-soi/${j}zp33ny.xls
done

cd ../..

# Get zip code polygons
DATADIR='data/zipcodes'
mkdir -p $DATADIR
cd $DATADIR
wget https://data.cityofnewyork.us/download/i8iw-xf4u/application%2Fzip -O zipcodes.zip
unzip zipcodes.zip
rm zipcodes.zip

cd ../..
