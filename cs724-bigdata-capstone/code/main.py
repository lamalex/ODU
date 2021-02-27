#/usr/bin/env python3

import os
import sys
import math
import time
import pickle
import dateparser
import collections
import geopandas as gpd
from ast import literal_eval
from datetime import datetime
from shapely.geometry import Point

from pyspark.sql import SparkSession
from pyspark import SparkContext, AccumulatorParam

# The following computation geometry algorithms were taken from
# CLRS CH. 33
def direction(pi, pj, pk):
    v = (pj.x - pi.x) * (pk.y - pi.y) - (pk.x - pi.x) * (pj.y - pi.y)
    return v

def onSegment(pi, pj, pk):
    print('checking on segment')
    if min(pi.x, pj.x) <= pk.x <= max(pi.x, pj.x) and min(pi.y, pj.y) <= pk.y <= max(pi.y, pj.y):
        return True
    return False

def segmentsIntersect(p1, p2, p3, p4):
    d1 = direction(p3, p4, p1)
    d2 = direction(p3, p4, p2)
    d3 = direction(p1, p2, p3)
    d4 = direction(p1, p2, p4)
    if ((d1 > 0 and d2 < 0) or (d1 < 0 and d2 > 0)) and ((d3 > 0 and d4 < 0) or (d3 < 0 and d4 > 0)):
        return True
    elif d1 == 0 and onSegment(p3, p4, p1):
        return True
    elif d2 == 0 and onSegment(p3, p4, p2):
        return True
    elif d3 == 0 and onSegment(p1, p2, p3):
        return True
    elif d4 == 0 and onSegment(p1, p2, p4):
        return True
    else:
        return False

def pointInside(row, point):
    # This seems wildly cumbersome to just get the points of our polygon in python
    # native types, but this is the best I could do.
    intersectCount = 0
    x,y = row.geometry.to_list()[0].exterior.coords.xy
    bounds = zip(x,y)
    for i in range(1, len(bounds)):
        p1 = Point(bounds[i - 1])
        p2 = Point(bounds[i])
        p3 = point
        p4 = Point(point.x, math.inf)
        if segmentsIntersect(p1, p2, p3, p4):
            intersectCount = intersectCount + 1

    # Ray Crossing algorithm: count the number of times a ray extending
    # from Point to +Inf intersects with a side of our polygon.
    # If that count is even (or 0) then the point is outside the polygon.
    # If the count is odd, then the point is inside.
    # https://en.wikipedia.org/wiki/Point_in_polygon
    return intersectCount & 1

class DictParam(AccumulatorParam):
    '''
    Adds a new key to a dictionary by recursively descending the dictionary
    hierarchy until d does not contain a key in u.
    u is expected to be a single entry like {"zipcode": {"yearmonthday": 1}}
    So that if "zipcode" does not exist in d then _update will add the key with
    sub dictionary. If "zipcode" does exist _update will either add a new monthdayyear
    value, or add v and return d
    '''
    def _update(self, d, u):
        if d is None:
            d = {}
        if u is None:
            return d
        for k, v in u.items():
            if isinstance(v, collections.Mapping):
                # The default {} seems to not always work,
                # so the nonetype check was added when get returns default None
                # for a missing key
                d[k] = self._update(d.get(k, {}), v)
            else:
                totalTrips = d.get(k)
                if totalTrips is None:
                    totalTrips = 0
                d[k] = totalTrips + v
        return d

    def zero(self, value = ""):
        return dict()

    def addInPlace(self, value1, value2):
        return self._update(value1, value2)

class Station:
    stationId = None
    zipcode = None
    def __init__(self, stationId, lon, lat):
        self.stationId = stationId
        self.zipcode = self.findZipcode(stationId, lon, lat)

    def findZipcode(self, stationId, lon, lat):
        global stationZips

        if stationId in stationZips.keys():
            return stationZips[stationId]

        try:
            lon = literal_eval(lon)
            lat = literal_eval(lat)
        except ValueError:
            return None

        p = Point(lon, lat)
        # this should not return greater than 1 since a point can only exist
        # in a single zipcode
        matches = zb.value[zb.value.contains(p)]
        if len(matches == 1):
            # hack around the fact that matches is a GeoSeries object
            # and there's no obvious way to get just the value
            zip = str(int(matches.ZIPCODE))
            stationZips[stationId] = zip
            return zip

        # This is only reached in an error condition where p is not in NYC,
        # or is somehow in multiple NYC zipcodes.
        #sys.stderr.write("The point %5f, %5f " % (lon, lat))
        if len(matches) == 0:
            #sys.stderr.write("was not located insize of an NYC zipcode\n")
            stationZips[stationId] = None
            return None
        else:
            #sys.stderr.write("was somehow located inside multiple zipcodes\n")
            stationZips[stationId] = None
            return None

class Trip:
    startStation = None
    endStation = None

    def __init__(self, row):
        self.startStation = Station(row['start station id'], row['start station longitude'], row['start station latitude'])
        self.endStation = Station(row['end station id'], row['end station longitude'], row['end station latitude'])

        self.startTime = self._parseDate(row['starttime'])
        self.endTime = self._parseDate(row['stoptime'])

    def crossesZipcode(self):
        return None not in [self.startStation.zipcode, self.endStation.zipcode] \
            and self.startStation.zipcode != self.endStation.zipcode

    def _parseDate(self, time):
        date_out_format = "%Y%m"

        return dateparser.parse(time).strftime(date_out_format)
        date_in_formats = ["%Y-%m-%d %H:%M%S", "%m/%d/%Y %H:%M:%S"]

        for format in date_in_formats:
            try:
                d = datetime.strptime(time, format)
                return d.strftime(date_out_format)
            except:
                continue
        return None

def countTrip(row):
    t = Trip(row)
    if t.crossesZipcode():
        global trip_counter
        d = { t.endStation.zipcode: { t.endTime: 1 }}
        trip_counter += d

        return row

#spark = SparkSession \
#    .builder \
#    .master("local") \
#    .appName("LauniCS724Citibike") \
#    .getOrCreate()

#sc = spark.sparkContext

stationZips = {}
trip_counter = sc.accumulator({}, DictParam())

shpfile = os.path.join('data', 'zipcodes', 'nyc_zip_code_polygons.shp')
zipcodes = gpd.read_file(shpfile)
zipcodes.to_crs(epsg = 4483, inplace = True)
zb = sc.broadcast(zipcodes)

start = time.monotonic()

csv = None
path = os.path.join('data', 'citibike')
for file in os.listdir(path):
    if not file.endswith('csv'):
        continue

    csvFilePath = os.path.join(path, file)
    rdd = sc.textFile(csvFilePath)
    csv2 = spark.read.csv(rdd, header = True)
    if csv == None:
        csv = csv2
    else:
        csv = csv.union(csv2)

# remove trips that start and end at the same station
csv = csv.filter(csv['start station id'] != csv['end station id'])
csv.cache()
# Sampling 1% which is approx 300,000 trips.
# Could not run problem on hadoop cluster to acctually
# parallize execution due to file system permissions.
# HPC lab was contacted twice, but no solution has been found.
#sample = csv.sample(fraction = , withReplacement = False)
#sample.cache()
crossings = csv.rdd.map(lambda row: countTrip(row))
#crossings = csv.rdd.map(lambda row: countTrip(row))
# Call count to force our map to run, updating the aggregator
num = crossings.count()
end = time.monotonic()

tstamp = datetime.now().strftime("%Y%m%d%H%M%S")
saveFile = "ziptrips.{}.pkl"
f = open(saveFile.format(tstamp), "wb")
pickle.dump(trip_counter.value, f)
f.close()

csv.registerTempTable('trips')
stations = spark.sql("SELECT DISTINCT `start station id` FROM trips")
bikes = spark.sql("SELECT DISTINCT `bikeid` FROM trips")
stations = stations.count()
bikes = bikes.count()

print("Execution took: %f s " % (end - start))
