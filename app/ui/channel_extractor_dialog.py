# -*- coding: utf-8 -*-
"""Channel Extractor Dialog for batch extracting video URLs from YouTube channels"""

from PySide6 import QtCore, QtGui, QtWidgets
from ui.channel_extractor_dialog_ui import Ui_ChannelExtractorDialog
from utils import ROOT
import sys
from pathlib import Path


class ChannelExtractorDialog(QtWidgets.QDialog, Ui_ChannelExtractorDialog):
    """Dialog to extract video URLs from YouTube channels"""

    # Signals
    urls_ready = QtCore.Signal(list, str)  # (urls, channel_name)

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setupUi(self)
        self.setWindowIcon(QtGui.QIcon(str(ROOT / "assets" / "yt-dlp-gui.ico")))

        self.extractor = None
        self.extracted_urls = []

        self.connect_signals()

    def connect_signals(self):
        """Connect button signals to slots"""
        self.extract_button.clicked.connect(self.start_extraction)
        self.stop_button.clicked.connect(self.stop_extraction)
        self.close_button.clicked.connect(self.close)

        self.copy_all_button.clicked.connect(self.copy_all_urls)
        self.copy_selected_button.clicked.connect(self.copy_selected_urls)
        self.add_all_button.clicked.connect(self.add_all_to_download)
        self.add_selected_button.clicked.connect(self.add_selected_to_download)

    def start_extraction(self):
        """Start extracting videos from the channel"""
        channel_url = self.channel_url_input.text().strip()

        if not channel_url:
            QtWidgets.QMessageBox.warning(self, "Input Error", "Please enter a channel URL")
            return

        if not self.is_valid_youtube_url(channel_url):
            QtWidgets.QMessageBox.warning(
                self, "Input Error", "Please enter a valid YouTube channel URL"
            )
            return

        # Clear previous results
        self.url_list.clear()
        self.extracted_urls = []
        self.channel_name_label.setText("Channel: Loading...")
        self.video_count_label.setText("Videos: 0")

        # Disable extract button, enable stop button
        self.extract_button.setEnabled(False)
        self.stop_button.setEnabled(True)
        self.channel_url_input.setEnabled(False)

        # Show progress bar
        self.progress_bar.setValue(0)
        self.progress_bar.setVisible(True)
        self.status_label.setText("Extracting videos...")

        # Import here to avoid circular imports
        from channel_extractor import ChannelExtractor

        # Create extractor with callbacks
        self.extractor = ChannelExtractor(
            on_progress=self.on_progress,
            on_complete=self.on_complete,
            on_error=self.on_error,
        )

        # Start extraction
        self.extractor.extract_urls_async(channel_url)

    def stop_extraction(self):
        """Stop the extraction process"""
        if self.extractor:
            self.extractor.stop()
        self.extract_button.setEnabled(True)
        self.stop_button.setEnabled(False)
        self.channel_url_input.setEnabled(True)
        self.progress_bar.setVisible(False)
        self.status_label.setText("Extraction stopped")

    def on_progress(self, current, total, message):
        """Update progress"""
        if total > 0:
            progress = int((current / total) * 100)
            self.progress_bar.setValue(progress)
        self.status_label.setText(message)

    def on_complete(self, urls, channel_name):
        """Handle extraction completion"""
        self.extracted_urls = urls
        self.url_list.clear()

        for url in urls:
            self.url_list.addItem(url)

        self.channel_name_label.setText(f"Channel: {channel_name}")
        self.video_count_label.setText(f"Videos: {len(urls)}")

        # Enable buttons
        self.extract_button.setEnabled(True)
        self.stop_button.setEnabled(False)
        self.channel_url_input.setEnabled(True)
        self.copy_all_button.setEnabled(True)
        self.copy_selected_button.setEnabled(True)
        self.add_all_button.setEnabled(True)
        self.add_selected_button.setEnabled(True)

        self.progress_bar.setVisible(False)
        self.status_label.setText(f"Successfully extracted {len(urls)} videos")

    def on_error(self, error_message):
        """Handle extraction error"""
        self.extract_button.setEnabled(True)
        self.stop_button.setEnabled(False)
        self.channel_url_input.setEnabled(True)
        self.progress_bar.setVisible(False)
        self.status_label.setText(f"Error: {error_message}")

        QtWidgets.QMessageBox.critical(self, "Extraction Error", error_message)

    def copy_all_urls(self):
        """Copy all URLs to clipboard"""
        if not self.extracted_urls:
            return

        clipboard = QtWidgets.QApplication.clipboard()
        clipboard.setText("\n".join(self.extracted_urls))
        self.status_label.setText(f"Copied {len(self.extracted_urls)} URLs to clipboard")

    def copy_selected_urls(self):
        """Copy selected URLs to clipboard"""
        selected_items = self.url_list.selectedItems()
        if not selected_items:
            QtWidgets.QMessageBox.warning(self, "No Selection", "Please select URLs to copy")
            return

        urls = [item.text() for item in selected_items]
        clipboard = QtWidgets.QApplication.clipboard()
        clipboard.setText("\n".join(urls))
        self.status_label.setText(f"Copied {len(urls)} URLs to clipboard")

    def add_all_to_download(self):
        """Add all URLs to download queue"""
        if not self.extracted_urls:
            return

        # Send signal that parent can connect to
        self.urls_ready.emit(self.extracted_urls, self.channel_name_label.text())
        self.status_label.setText(f"Added {len(self.extracted_urls)} videos to download queue")

    def add_selected_to_download(self):
        """Add selected URLs to download queue"""
        selected_items = self.url_list.selectedItems()
        if not selected_items:
            QtWidgets.QMessageBox.warning(self, "No Selection", "Please select URLs to add")
            return

        urls = [item.text() for item in selected_items]
        self.urls_ready.emit(urls, "Selected Videos")
        self.status_label.setText(f"Added {len(urls)} videos to download queue")

    @staticmethod
    def is_valid_youtube_url(url: str) -> bool:
        """Check if the URL is a valid YouTube URL"""
        return (
            "youtube.com" in url.lower()
            or "youtu.be" in url.lower()
        )

    def closeEvent(self, event):
        """Handle window close event"""
        if self.extractor and self.extractor.is_running:
            reply = QtWidgets.QMessageBox.question(
                self,
                "Extraction in Progress",
                "Extraction is still running. Do you want to stop and close?",
                QtWidgets.QMessageBox.StandardButton.Yes | QtWidgets.QMessageBox.StandardButton.No,
            )
            if reply == QtWidgets.QMessageBox.StandardButton.Yes:
                self.extractor.stop()
                event.accept()
            else:
                event.ignore()
        else:
            event.accept()
