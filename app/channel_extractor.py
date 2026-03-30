# -*- coding: utf-8 -*-
"""YouTube channel video URL extractor using yt-dlp"""

import json
import subprocess as sp
import sys
from typing import List, Tuple
from threading import Thread
import logging

logger = logging.getLogger(__name__)


class ChannelExtractor:
    """Extract video URLs from YouTube channels"""

    def __init__(self, on_progress=None, on_complete=None, on_error=None):
        """
        Initialize the extractor with callback functions.

        Args:
            on_progress: Callback function(current, total, message) for progress updates
            on_complete: Callback function(urls, channel_name) when extraction is complete
            on_error: Callback function(error_message) on error
        """
        self.on_progress = on_progress
        self.on_complete = on_complete
        self.on_error = on_error
        self.is_running = False

    def extract_urls(self, channel_url: str) -> Tuple[List[str], str]:
        """
        Extract video URLs from a YouTube channel.

        Args:
            channel_url: YouTube channel URL

        Returns:
            Tuple of (video_urls_list, channel_name)
        """
        video_urls = []
        channel_name = ""

        try:
            if self.on_progress:
                self.on_progress(0, 100, "Extracting channel information...")

            # Use yt-dlp command to extract flat playlist
            create_window = sp.CREATE_NO_WINDOW if sys.platform == "win32" else 0
            cmd = [
                "yt-dlp",
                "--flat-playlist",
                "--dump-json",
                "--no-warnings",
                channel_url
            ]

            result = sp.run(
                cmd,
                capture_output=True,
                text=True,
                creationflags=create_window
            )

            if result.returncode != 0:
                raise Exception(f"yt-dlp failed: {result.stderr}")

            # Parse JSON output line by line
            lines = result.stdout.strip().split('\n')
            total = len(lines)

            for idx, line in enumerate(lines):
                if not line.strip():
                    continue

                try:
                    entry = json.loads(line)

                    # Get channel name from first entry
                    if idx == 0 and 'channel' in entry:
                        channel_name = entry['channel']
                    elif idx == 0 and 'uploader' in entry:
                        channel_name = entry['uploader']

                    # Extract video URL
                    if 'url' in entry:
                        video_urls.append(entry['url'])
                    elif 'id' in entry:
                        video_urls.append(f"https://www.youtube.com/watch?v={entry['id']}")
                    elif 'webpage_url' in entry:
                        video_urls.append(entry['webpage_url'])

                    if self.on_progress:
                        progress = int((idx + 1) / total * 100)
                        self.on_progress(idx + 1, total, f"Extracted {idx + 1}/{total} videos")

                except json.JSONDecodeError:
                    logger.warning(f"Failed to parse JSON line: {line}")
                    continue

            if not channel_name:
                channel_name = "Unknown Channel"

        except Exception as e:
            error_msg = f"Error extracting channel: {str(e)}"
            logger.error(error_msg)
            if self.on_error:
                self.on_error(error_msg)
            raise

        return video_urls, channel_name

    def extract_urls_async(self, channel_url: str):
        """
        Extract video URLs asynchronously.

        Args:
            channel_url: YouTube channel URL
        """
        def worker():
            self.is_running = True
            try:
                urls, name = self.extract_urls(channel_url)
                if self.on_complete:
                    self.on_complete(urls, name)
            finally:
                self.is_running = False

        thread = Thread(target=worker, daemon=True)
        thread.start()

    def stop(self):
        """Stop the extraction process"""
        self.is_running = False
