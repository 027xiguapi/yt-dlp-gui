import logging
import sys

import qtawesome as qta
from dep_dl import DepWorker
from PySide6 import QtCore, QtGui, QtWidgets
from ui.main_window import Ui_MainWindow
from ui.sniffer_dialog import SnifferDialog
from ui.youtube_browser_dialog import YoutubeBrowserDialog
from ui.channel_extractor_dialog import ChannelExtractorDialog
from utils import BIN_DIR, ROOT, ItemRoles, load_toml, save_toml
from worker import DownloadWorker

__version__ = ""
logging.basicConfig(
    level=logging.DEBUG,
    format="%(asctime)s %(levelname)s (%(module)s:%(lineno)d) %(message)s",
    handlers=[
        logging.FileHandler(ROOT / "debug.log", encoding="utf-8", delay=True),
        logging.StreamHandler(),
    ],
)
logger = logging.getLogger(__name__)


class MainWindow(QtWidgets.QMainWindow, Ui_MainWindow):
    def __init__(self):
        super().__init__()
        self.setupUi(self)
        self.setWindowIcon(QtGui.QIcon(str(ROOT / "assets" / "yt-dlp-gui.ico")))
        self.pb_add.setIcon(qta.icon("mdi6.plus"))
        self.pb_add.setIconSize(QtCore.QSize(21, 21))
        self.pb_clear.setIcon(qta.icon("mdi6.trash-can-outline"))
        self.pb_clear.setIconSize(QtCore.QSize(22, 22))
        self.pb_download.setIcon(qta.icon("mdi6.download"))
        self.pb_download.setIconSize(QtCore.QSize(22, 22))
        self.te_link.setPlaceholderText(
            "https://www.youtube.com/watch?v=hTWKbfoikeg\n"
            "https://www.youtube.com/watch?v=KQetemT1sWc\n"
            "https://www.youtube.com/watch?v=yKNxeF4KMsY"
        )

        self.tw.setColumnWidth(0, 200)
        self.te_link.setFocus()
        
        try:
            self.load_config()
        except Exception:
            # 如果加载配置失败，直接返回，不继续初始化
            return

        self.connect_ui()
        self.pb_download.setEnabled(False)
        self.show()

        self.dep_worker = DepWorker(self.config["general"]["update_ytdlp"])
        self.dep_worker.finished.connect(self.on_dep_finished)
        self.dep_worker.progress.connect(self.on_dep_progress)
        self.dep_worker.start()

        self.to_dl = {}
        self.workers = {}
        self.index = 0

        self.tw.setContextMenuPolicy(QtCore.Qt.ContextMenuPolicy.CustomContextMenu)
        self.tw.customContextMenuRequested.connect(self.open_menu)

    def connect_ui(self):
        # buttons
        self.pb_path.clicked.connect(self.button_path)
        if hasattr(self, 'pb_cookie'):
            self.pb_cookie.clicked.connect(self.button_cookie)
            logger.info("Cookie button connected successfully")
        else:
            logger.warning("pb_cookie button not found in UI")
        self.pb_add.clicked.connect(self.button_add)
        self.pb_clear.clicked.connect(self.button_clear)
        self.pb_download.clicked.connect(self.button_download)

        # menu bar
        self.action_open_bin_folder.triggered.connect(lambda: self.open_folder(BIN_DIR))
        self.action_open_log_folder.triggered.connect(lambda: self.open_folder(ROOT))
        self.action_exit.triggered.connect(self.close)
        self.action_about.triggered.connect(self.show_about)
        self.action_clear_url_list.triggered.connect(self.te_link.clear)
        self.action_browser_sniffer.triggered.connect(self.show_browser_sniffer)

        # YouTube browser
        self.action_youtube_browser.triggered.connect(self.show_youtube_browser)

        # YouTube channel extractor
        if hasattr(self, 'action_channel_extractor'):
            self.action_channel_extractor.triggered.connect(self.show_channel_extractor)

    def on_dep_progress(self, status):
        self.statusBar.showMessage(status, 10000)

    def on_dep_finished(self):
        self.dep_worker.deleteLater()
        self.pb_download.setEnabled(True)

    def open_folder(self, path):
        QtGui.QDesktopServices.openUrl(QtCore.QUrl.fromLocalFile(path))

    def show_about(self):
        QtWidgets.QMessageBox.about(
            self,
            "About yt-dlp-gui",
            f'<a href="https://github.com/dsymbol/yt-dlp-gui">yt-dlp-gui</a> {__version__}<br><br>'
            "A GUI for yt-dlp written in PySide6.",
        )
    
    def show_browser_sniffer(self):
        """显示浏览器抓包工具对话框"""
        dialog = SnifferDialog(self)
        dialog.exec()
    
    def show_youtube_browser(self):
        """显示 YouTube 浏览器"""
        dialog = YoutubeBrowserDialog(self)
        dialog.exec()

    def show_channel_extractor(self):
        """Show YouTube channel extractor dialog"""
        dialog = ChannelExtractorDialog(self)
        dialog.urls_ready.connect(self.add_urls_from_channel)
        dialog.exec()

    def add_urls_from_channel(self, urls, channel_name):
        """Add URLs extracted from channel to download list"""
        urls_text = "\n".join(urls)
        self.te_link.clear()
        self.te_link.appendPlainText(urls_text)
        self.statusBar.showMessage(f"Added {len(urls)} videos from {channel_name}", 5000)

    def add_url_to_download(self, url):
        """添加 URL 到下载列表"""
        # 检查 URL 是否有效
        if url and ("youtube.com" in url or "youtu.be" in url):
            # 清除 URL 输入框并添加新 URL
            self.te_link.clear()
            self.te_link.appendPlainText(url)
            # 自动添加到下载列表
            self.button_add()

    def open_menu(self, position):
        menu = QtWidgets.QMenu()

        delete_action = menu.addAction(qta.icon("mdi6.trash-can"), "Delete")
        copy_url_action = menu.addAction(qta.icon("mdi6.content-copy"), "Copy URL")
        open_folder_action = menu.addAction(qta.icon("mdi6.folder-open"), "Open Folder")

        item = self.tw.itemAt(position)

        if item:
            item_path = item.data(0, ItemRoles.PathRole)
            item_link = item.data(0, ItemRoles.LinkRole)
            action = menu.exec(self.tw.viewport().mapToGlobal(position))

            if action == delete_action:
                self.remove_item(item, 0)
            elif action == copy_url_action:
                QtWidgets.QApplication.clipboard().setText(item_link)
                logger.info(f"Copied URL to clipboard: {item_link}")
            elif action == open_folder_action:
                self.open_folder(item_path)
                logger.info(f"Opened folder: {item_path}")

    def remove_item(self, item, column):
        item_id = item.data(0, ItemRoles.IdRole)
        item_text = item.text(0)

        logger.debug(f"Removing download ({item_id}): {item_text}")

        if worker := self.workers.get(item_id):
            worker.stop()

        self.to_dl.pop(item_id, None)
        self.tw.takeTopLevelItem(
            self.tw.indexOfTopLevelItem(item)
        )  # remove and return a top-level item

    def button_path(self):
        path = QtWidgets.QFileDialog.getExistingDirectory(
            self,
            "Select a folder",
            self.le_path.text() or QtCore.QDir.homePath(),
            QtWidgets.QFileDialog.Option.ShowDirsOnly,
        )

        if path:
            self.le_path.setText(path)

    def button_cookie(self):
        logger.info("Cookie button clicked")
        file_path, _ = QtWidgets.QFileDialog.getOpenFileName(
            self,
            "Select cookies file",
            self.le_cookie.text() or QtCore.QDir.homePath(),
            "Cookie files (*.txt);;All files (*.*)",
        )

        if file_path:
            self.le_cookie.setText(file_path)
            logger.info(f"Cookie file selected: {file_path}")

    def button_add(self):
        missing = []
        preset = self.dd_preset.currentText()
        links = self.te_link.toPlainText()
        path = self.le_path.text()

        if not links:
            missing.append("Video URL")
        if not path:
            missing.append("Save to")

        if missing:
            missing_fields = ", ".join(missing)
            return QtWidgets.QMessageBox.information(
                self,
                "Application Message",
                f"Required field{'s' if len(missing) > 1 else ''} ({missing_fields}) missing.",
            )

        self.te_link.clear()

        for link in links.split("\n"):
            link = link.strip()
            item = QtWidgets.QTreeWidgetItem(
                self.tw, [link, preset, "-", "", "Queued", "-", "-"]
            )
            pb = QtWidgets.QProgressBar()
            pb.setStyleSheet("QProgressBar { margin-bottom: 3px; }")
            pb.setTextVisible(False)
            self.tw.setItemWidget(item, 3, pb)
            [
                item.setTextAlignment(i, QtCore.Qt.AlignmentFlag.AlignCenter)
                for i in range(1, 6)
            ]
            item.setData(0, ItemRoles.IdRole, self.index)
            item.setData(0, ItemRoles.LinkRole, link)
            item.setData(0, ItemRoles.PathRole, path)

            cookie_path = self.le_cookie.text().strip() or None
            worker = DownloadWorker(item, self.config, link, path, preset, cookie_path)
            self.to_dl[self.index] = worker
            logger.info(f"Queued download ({self.index}) added {link}")
            self.index += 1

    def button_clear(self):
        if self.workers:
            return QtWidgets.QMessageBox.critical(
                self,
                "Application Message",
                "Unable to clear list because there are active downloads in progress.\n"
                "Remove a download by right clicking on it and selecting delete.",
            )

        self.workers = {}
        self.to_dl = {}
        self.tw.clear()

    def button_download(self):
        if self.te_link.toPlainText().strip():
            self.button_add()

        if not self.to_dl:
            return QtWidgets.QMessageBox.information(
                self,
                "Application Message",
                "Unable to download because there are no links in the list.",
            )

        for idx, worker in self.to_dl.items():
            self.workers[idx] = worker
            worker.finished.connect(worker.deleteLater)
            worker.finished.connect(lambda x=idx: self.workers.pop(x))
            worker.progress.connect(self.on_dl_progress)
            worker.start()

        self.to_dl = {}

    def load_config(self):
        config_path = ROOT / "config.toml"

        try:
            self.config = load_toml(config_path)
        except Exception:
            QtWidgets.QMessageBox.critical(
                self,
                "Application Message",
                "Config file error.",
            )
            logger.error("Config file error.", exc_info=True)
            raise  # 抛出异常而不是直接退出

        update_ytdlp = self.config["general"].get("update_ytdlp")
        self.config["general"]["update_ytdlp"] = update_ytdlp if update_ytdlp else True
        self.dd_preset.addItems(self.config["presets"].keys())
        self.dd_preset.setCurrentIndex(self.config["general"]["current_preset"])
        self.le_path.setText(self.config["general"]["path"])
        self.le_cookie.setText(self.config["general"].get("cookie_path", ""))

    def closeEvent(self, event):
        self.config["general"]["current_preset"] = self.dd_preset.currentIndex()
        self.config["general"]["path"] = self.le_path.text()
        self.config["general"]["cookie_path"] = self.le_cookie.text()
        save_toml(ROOT / "config.toml", self.config)
        event.accept()

    def on_dl_progress(self, item: QtWidgets.QTreeWidgetItem, emit_data):
        try:
            for data in emit_data:
                index, update = data
                if index != 3:
                    item.setText(index, update)
                else:
                    pb = self.tw.itemWidget(item, index)
                    pb.setValue(round(float(update.replace("%", ""))))
        except AttributeError:
            logger.info(f"Download ({item.data(0, ItemRoles.IdRole)}) no longer exists")


if __name__ == "__main__":
    app = QtWidgets.QApplication(sys.argv)
    window = MainWindow()
    sys.exit(app.exec())
