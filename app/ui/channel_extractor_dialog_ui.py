# -*- coding: utf-8 -*-
"""Auto-generated UI for Channel Extractor Dialog"""

from PySide6.QtCore import (QCoreApplication, QMetaObject, QRect, QSize)
from PySide6.QtGui import QFont
from PySide6.QtWidgets import (QDialog, QVBoxLayout, QHBoxLayout, QGroupBox,
    QGridLayout, QLabel, QLineEdit, QPushButton, QProgressBar, QListWidget,
    QListWidgetItem)


class Ui_ChannelExtractorDialog(object):
    def setupUi(self, ChannelExtractorDialog):
        if not ChannelExtractorDialog.objectName():
            ChannelExtractorDialog.setObjectName(u"ChannelExtractorDialog")
        ChannelExtractorDialog.resize(900, 600)

        self.verticalLayout = QVBoxLayout(ChannelExtractorDialog)
        self.verticalLayout.setObjectName(u"verticalLayout")

        # Channel URL input
        url_layout = QHBoxLayout()
        self.url_label = QLabel(ChannelExtractorDialog)
        self.url_label.setObjectName(u"url_label")
        self.url_label.setText("Channel URL:")
        url_layout.addWidget(self.url_label)

        self.channel_url_input = QLineEdit(ChannelExtractorDialog)
        self.channel_url_input.setObjectName(u"channel_url_input")
        self.channel_url_input.setPlaceholderText(u"e.g., https://www.youtube.com/@NBA/videos")
        url_layout.addWidget(self.channel_url_input)
        self.verticalLayout.addLayout(url_layout)

        # Button layout
        button_layout = QHBoxLayout()
        self.extract_button = QPushButton(ChannelExtractorDialog)
        self.extract_button.setObjectName(u"extract_button")
        self.extract_button.setText(u"Extract Videos")
        button_layout.addWidget(self.extract_button)

        self.stop_button = QPushButton(ChannelExtractorDialog)
        self.stop_button.setObjectName(u"stop_button")
        self.stop_button.setText(u"Stop")
        self.stop_button.setEnabled(False)
        button_layout.addWidget(self.stop_button)
        button_layout.addStretch()
        self.verticalLayout.addLayout(button_layout)

        # Progress bar
        self.progress_bar = QProgressBar(ChannelExtractorDialog)
        self.progress_bar.setObjectName(u"progress_bar")
        self.progress_bar.setVisible(False)
        self.verticalLayout.addWidget(self.progress_bar)

        # Status label
        self.status_label = QLabel(ChannelExtractorDialog)
        self.status_label.setObjectName(u"status_label")
        self.status_label.setText(u"Ready")
        self.verticalLayout.addWidget(self.status_label)

        # Results group
        self.results_group = QGroupBox(ChannelExtractorDialog)
        self.results_group.setObjectName(u"results_group")
        self.results_group.setTitle(u"Extracted Videos")
        results_layout = QVBoxLayout(self.results_group)

        # Channel info
        info_layout = QHBoxLayout()
        self.channel_name_label = QLabel(ChannelExtractorDialog)
        self.channel_name_label.setObjectName(u"channel_name_label")
        self.channel_name_label.setText(u"Channel: Not loaded")
        info_layout.addWidget(self.channel_name_label)

        self.video_count_label = QLabel(ChannelExtractorDialog)
        self.video_count_label.setObjectName(u"video_count_label")
        self.video_count_label.setText(u"Videos: 0")
        info_layout.addWidget(self.video_count_label)
        info_layout.addStretch()
        results_layout.addLayout(info_layout)

        # URL list
        self.url_list = QListWidget(ChannelExtractorDialog)
        self.url_list.setObjectName(u"url_list")
        results_layout.addWidget(self.url_list)

        # List actions
        list_action_layout = QHBoxLayout()
        self.copy_all_button = QPushButton(ChannelExtractorDialog)
        self.copy_all_button.setObjectName(u"copy_all_button")
        self.copy_all_button.setText(u"Copy All URLs")
        self.copy_all_button.setEnabled(False)
        list_action_layout.addWidget(self.copy_all_button)

        self.copy_selected_button = QPushButton(ChannelExtractorDialog)
        self.copy_selected_button.setObjectName(u"copy_selected_button")
        self.copy_selected_button.setText(u"Copy Selected")
        self.copy_selected_button.setEnabled(False)
        list_action_layout.addWidget(self.copy_selected_button)

        self.add_all_button = QPushButton(ChannelExtractorDialog)
        self.add_all_button.setObjectName(u"add_all_button")
        self.add_all_button.setText(u"Add All to Download")
        self.add_all_button.setEnabled(False)
        list_action_layout.addWidget(self.add_all_button)

        self.add_selected_button = QPushButton(ChannelExtractorDialog)
        self.add_selected_button.setObjectName(u"add_selected_button")
        self.add_selected_button.setText(u"Add Selected to Download")
        self.add_selected_button.setEnabled(False)
        list_action_layout.addWidget(self.add_selected_button)

        results_layout.addLayout(list_action_layout)
        self.verticalLayout.addWidget(self.results_group)

        # Dialog buttons
        dialog_button_layout = QHBoxLayout()
        dialog_button_layout.addStretch()
        self.close_button = QPushButton(ChannelExtractorDialog)
        self.close_button.setObjectName(u"close_button")
        self.close_button.setText(u"Close")
        dialog_button_layout.addWidget(self.close_button)
        self.verticalLayout.addLayout(dialog_button_layout)

        QMetaObject.connectSlotsByName(ChannelExtractorDialog)
