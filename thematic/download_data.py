"""
Fetches all scripts from imscb.com and writes them to disk
CS620
Semester project
@author: Alex Launi
"""
import os
import logging
from typing import Optional
from collections import deque
from urllib.parse import urlparse, urlunparse
import asyncio
import aiohttp
import aiofiles
from bs4 import BeautifulSoup
from constants import DATA_DIR


class RateLimitingSemaphore:
    """
    Attempts to rate limit API requests so that we don't get kicked off the server.
    This is from StackOverflow, https://stackoverflow.com/questions/38683243/asyncio-rate-limiting
    Tried with just using asyncio.Semaphore but didn't appear to be working as expected.
    """

    def __init__(self, qps_limit, loop=None):
        self.loop = loop or asyncio.get_event_loop()
        self.qps_limit = qps_limit

        # The number of calls that are queued up, waiting for their turn.
        self.queued_calls = 0

        # The times of the last N executions, where N=qps_limit -
        # this should allow us to calculate the QPS within the
        # last ~ second. Note that this also allows us to schedule the first N
        # executions immediately.
        self.call_times = deque()

    async def __aenter__(self):
        self.queued_calls += 1
        while True:
            cur_rate = 0
            if len(self.call_times) == self.qps_limit:
                cur_rate = len(self.call_times) / \
                    (self.loop.time() - self.call_times[0])
            if cur_rate < self.qps_limit:
                break
            interval = 1. / self.qps_limit
            elapsed_time = self.loop.time() - self.call_times[-1]
            await asyncio.sleep(self.queued_calls * interval - elapsed_time)
        self.queued_calls -= 1

        if len(self.call_times) == self.qps_limit:
            self.call_times.popleft()
        self.call_times.append(self.loop.time())

    async def __aexit__(self, exc_type, exc, tb):
        pass


async def do_the_fetch(semaphore, session, url) -> Optional[bytes]:
    """
    D.R.Y. Take a url and a session, get the url and return the future.
    """
    async with semaphore:
        try:
            async with session.get(url) as response:
                if response.status != 200:
                    logging.warning(
                        "%s was unavailable: HTTP status code %d",
                        url,
                        response.status)
                    return None
                try:
                    return await response.text()
                except Exception as err:
                    logging.error(
                        "Ooops! something went horribly wrong %s", err)
        except aiohttp.client_exceptions.ServerDisconnectedError:
            logging.warning("Recieved disconnect from server fetching %s" % url)
            decreased_limit = semaphore.qps_limit - semaphore.qps_limit * 0.2
            semaphore.qps_limit = min(2, decreased_limit)
            do_the_fetch(semaphore, session, url)


def load_html(raw_http) -> BeautifulSoup:
    return BeautifulSoup(raw_http, 'html.parser')


def extract_title(url):
    """
    Strips URL path and extension from url
    """
    url_path = urlparse(url).path
    resource = os.path.basename(url_path)
    return os.path.splitext(resource)[0]


class ScriptDownloader:
    """
    Given the url of a script's home page (which has link to script, reviews, etc)
    Pull out the url of the actual script's text, download that text, and write it to disk.
    """

    def __init__(
            self,
            semaphore: RateLimitingSemaphore,
            session: aiohttp.ClientSession,
            script_url: str,
            output_dir: str):
        self.semaphore = semaphore
        self.session = session
        self.url = script_url
        self.movie_title = extract_title(script_url)

        filename = self.movie_title + ".txt"
        self.output_path = os.path.join(output_dir, filename)

    async def download(self, force=False):
        """
        Asyncronously download the script data from url and write the file to disk
        """
        should_skip_download = not force and os.path.exists(self.output_path)
        if should_skip_download:
            logging.info("Skipping download of %s", self.movie_title)
            return

        logging.info("Downloading %s from %s", self.movie_title, self.url)
        script_url = await self._extract_script_url()
        if script_url is None:
            return
        script_text = await self._download_script(script_url)
        if script_text is None:
            return
        await self._write_script_to_disk(script_text)

    async def _extract_script_url(self) -> Optional[str]:
        """
        Pull the url of the actual script from the script's homepage
        May return None if a status code other than 200 is returned
        """
        text = await do_the_fetch(self.semaphore, self.session, self.url)
        if text is None:
            return None

        urls = load_html(text).select('.script-details a[href*="/scripts/"]')
        if len(urls) == 0:
            logging.warning("No candidate scripts for %s", self.movie_title)
            return None
        elif len(urls) > 1:
            logging.warning(
                "Multiple candidate scripts for %s. Returning the first. Double check your data.",
                self.url)

        resource_path = urls.pop().get("href")
        parts = urlparse(self.url)
        full_url = urlunparse(
            (parts.scheme, parts.netloc, resource_path, None, None, None))
        logging.info("Found script url: %s", full_url)
        return full_url

    async def _download_script(self, url) -> Optional[str]:
        """
        Download the script and return the text
        May return None if a status code other than 200 is returned
        """
        text = await do_the_fetch(self.semaphore, self.session, url)
        if text is None:
            logging.warning("No text was returned from %s", url)
            return None

        return load_html(text).select_one('.scrtext').text

    async def _write_script_to_disk(self, script_text):
        """
        Outputs text to disk at self.output_path
        """
        async with aiofiles.open(self.output_path, 'w') as script_file:
            logging.info("Writing %s", self.output_path)
            await script_file.write(script_text)


async def fetch_all_scripts(sempahore: RateLimitingSemaphore, session: aiohttp.ClientSession, url: str):
    """
    Fetches list of scripts and passes them to downloader
    """
    text = await do_the_fetch(sempahore, session, url)
    if text is None:
        yield None

    for script_url in load_html(text).select('a[href*="/Movie Scripts/"]'):
        try:
            full_url = "{0}{1}".format(
                'https://www.imsdb.com', script_url["href"])
            yield full_url
        except KeyError:
            logging.warning("No href attribute on %s", script_url)
            continue


async def run(qps):
    """
    Async main
    """
    scripts_url = 'https://www.imsdb.com/all%20scripts/'
    semaphore = RateLimitingSemaphore(qps)
    async with aiohttp.ClientSession() as session:
        downloaders = [ScriptDownloader(semaphore, session, script_url, DATA_DIR) async for script_url in fetch_all_scripts(semaphore, session, scripts_url)]
        tasks = map(lambda d: asyncio.ensure_future(d.download()), downloaders)
        await asyncio.wait(tasks)


def setup_logging():
    """
    Initialize logging system at desired level
    """
    logging.basicConfig(level=logging.INFO,
                        format='%(asctime)s %(levelname)-8s %(message)s',
                        datefmt='%m-%d %H:%M')
    console = logging.StreamHandler()
    console.setLevel(logging.INFO)
    formatter = logging.Formatter('%(name)-12s: %(levelname)-8s %(message)s')
    console.setFormatter(formatter)
    logging.getLogger('').addHandler(console)


def main():
    """
    Syncronous entry point
    """
    setup_logging()
    loop = asyncio.get_event_loop()
    qps = 15
    loop.run_until_complete(run(qps))

if __name__ == '__main__':
    main()
