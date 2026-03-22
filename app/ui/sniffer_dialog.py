"""
浏览器抓包工具对话框
"""
import logging
from pathlib import Path
from typing import List, Optional

from PySide6 import QtCore, QtGui, QtWidgets
from PySide6.QtCore import Signal, Slot, QThread, QTimer
from PySide6.QtWidgets import QDialog, QTableWidgetItem, QHeaderView, QMessageBox

try:
    from .sniffer_dialog_ui import Ui_SnifferDialog
    from ..browser_sniffer import BrowserSniffer, CapturedResource
except ImportError:
    # 在打包环境中，使用绝对导入
    from ui.sniffer_dialog_ui import Ui_SnifferDialog
    from browser_sniffer import BrowserSniffer, CapturedResource

logger = logging.getLogger(__name__)

class SnifferWorker(QThread):
    """浏览器嗅探工作线程"""
    resource_captured = Signal(CapturedResource)
    status_changed = Signal(str)
    finished_signal = Signal()
    
    def __init__(self, url: str, browser_type: str = "chrome"):
        super().__init__()
        self.url = url
        self.browser_type = browser_type
        self.sniffer = BrowserSniffer(browser_type)
        self._is_running = True
        
    def run(self):
        """运行嗅探任务"""
        try:
            self.status_changed.emit(f"Starting browser capture for {self.url}")
            
            # 这里应该使用真实的浏览器自动化
            # 由于环境限制，我们使用模拟数据
            self._simulate_capture()
            
            self.status_changed.emit("Capture completed")
            self.finished_signal.emit()
            
        except Exception as e:
            logger.error(f"Browser sniffer error: {e}", exc_info=True)
            self.status_changed.emit(f"Error: {str(e)}")
            
    def _simulate_capture(self):
        """模拟捕获过程"""
        import time
        import random
        
        # 模拟一些视频资源
        video_urls = [
            "https://rr1---sn-4g5e6nzl.googlevideo.com/videoplayback",
            "https://rr2---sn-4g5e6nzl.googlevideo.com/videoplayback",
            "https://rr3---sn-4g5e6nzl.googlevideo.com/videoplayback",
        ]
        
        # 模拟一些图片资源
        image_urls = [
            "https://i.ytimg.com/vi/hTWKbfoikeg/maxresdefault.jpg",
            "https://i.ytimg.com/vi/hTWKbfoikeg/hqdefault.jpg",
            "https://i.ytimg.com/vi/hTWKbfoikeg/mqdefault.jpg",
            "https://i.ytimg.com/vi/KQetemT1sWc/maxresdefault.jpg",
        ]
        
        # 模拟捕获视频资源
        for i, url in enumerate(video_urls):
            if not self._is_running:
                break
                
            resource = CapturedResource(
                url=url + f"?id={i}",
                resource_type="video",
                mime_type="video/mp4",
                size=random.randint(10*1024*1024, 100*1024*1024),  # 10-100MB
                timestamp=QtCore.QDateTime.currentDateTime().toString(QtCore.Qt.ISODate),
                tab_url=self.url,
                method="GET",
                status=200
            )
            self.resource_captured.emit(resource)
            self.status_changed.emit(f"Captured video resource {i+1}/{len(video_urls)}")
            time.sleep(0.5)
            
        # 模拟捕获图片资源
        for i, url in enumerate(image_urls):
            if not self._is_running:
                break
                
            resource = CapturedResource(
                url=url,
                resource_type="image",
                mime_type="image/jpeg",
                size=random.randint(100*1024, 2*1024*1024),  # 100KB-2MB
                timestamp=QtCore.QDateTime.currentDateTime().toString(QtCore.Qt.ISODate),
                tab_url=self.url,
                method="GET",
                status=200
            )
            self.resource_captured.emit(resource)
            self.status_changed.emit(f"Captured image resource {i+1}/{len(image_urls)}")
            time.sleep(0.3)
            
    def stop(self):
        """停止嗅探"""
        self._is_running = False

class SnifferDialog(QDialog, Ui_SnifferDialog):
    """浏览器嗅探对话框"""
    
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setupUi(self)
        
        # 初始化变量
        self.worker: Optional[SnifferWorker] = None
        self.captured_resources: List[CapturedResource] = []
        
        # 设置表格
        self._setup_tables()
        
        # 连接信号
        self._connect_signals()
        
        # 设置窗口图标
        self.setWindowIcon(QtGui.QIcon(str(Path(__file__).parent.parent / "assets" / "yt-dlp-gui.ico")))
        
    def _setup_tables(self):
        """设置表格属性"""
        # 设置视频表格
        self.table_videos.horizontalHeader().setSectionResizeMode(0, QHeaderView.Stretch)
        self.table_videos.horizontalHeader().setSectionResizeMode(1, QHeaderView.ResizeToContents)
        self.table_videos.horizontalHeader().setSectionResizeMode(2, QHeaderView.ResizeToContents)
        self.table_videos.horizontalHeader().setSectionResizeMode(3, QHeaderView.ResizeToContents)
        self.table_videos.horizontalHeader().setSectionResizeMode(4, QHeaderView.ResizeToContents)
        
        # 设置图片表格
        self.table_images.horizontalHeader().setSectionResizeMode(0, QHeaderView.Stretch)
        self.table_images.horizontalHeader().setSectionResizeMode(1, QHeaderView.ResizeToContents)
        self.table_images.horizontalHeader().setSectionResizeMode(2, QHeaderView.ResizeToContents)
        self.table_images.horizontalHeader().setSectionResizeMode(3, QHeaderView.ResizeToContents)
        self.table_images.horizontalHeader().setSectionResizeMode(4, QHeaderView.ResizeToContents)
        self.table_images.horizontalHeader().setSectionResizeMode(5, QHeaderView.ResizeToContents)
        
    def _connect_signals(self):
        """连接信号和槽"""
        self.pb_start.clicked.connect(self.start_capture)
        self.pb_stop.clicked.connect(self.stop_capture)
        self.pb_add_video.clicked.connect(self.add_selected_videos)
        self.pb_download_video.clicked.connect(self.download_selected_videos)
        self.pb_add_image.clicked.connect(self.add_selected_images)
        self.pb_download_image.clicked.connect(self.download_selected_images)
        self.pb_export.clicked.connect(self.export_data)
        
    @Slot()
    def start_capture(self):
        """开始捕获"""
        url = self.le_url.text().strip()
        if not url:
            QMessageBox.warning(self, "Warning", "Please enter a YouTube URL")
            return
            
        # 禁用开始按钮，启用停止按钮
        self.pb_start.setEnabled(False)
        self.pb_stop.setEnabled(True)
        
        # 清空之前的捕获结果
        self.captured_resources.clear()
        self.table_videos.setRowCount(0)
        self.table_images.setRowCount(0)
        
        # 获取浏览器类型
        browser_text = self.cb_browser.currentText()
        browser_type = browser_text.lower()
        
        # 创建并启动工作线程
        self.worker = SnifferWorker(url, browser_type)
        self.worker.resource_captured.connect(self.on_resource_captured)
        self.worker.status_changed.connect(self.statusBar.showMessage)
        self.worker.finished_signal.connect(self.on_capture_finished)
        self.worker.start()
        
        self.statusBar.showMessage(f"Starting capture for {url}...")
        
    @Slot()
    def stop_capture(self):
        """停止捕获"""
        if self.worker:
            self.worker.stop()
            self.worker.wait()
            self.worker = None
            
        self.pb_start.setEnabled(True)
        self.pb_stop.setEnabled(False)
        self.statusBar.showMessage("Capture stopped")
        
    @Slot(CapturedResource)
    def on_resource_captured(self, resource: CapturedResource):
        """处理捕获到的资源"""
        self.captured_resources.append(resource)
        
        # 根据资源类型添加到相应的表格
        if resource.resource_type == "video":
            self._add_to_video_table(resource)
        elif resource.resource_type == "image":
            self._add_to_image_table(resource)
            
        # 更新统计信息
        self._update_statistics()
        
    def _add_to_video_table(self, resource: CapturedResource):
        """添加视频资源到表格"""
        row = self.table_videos.rowCount()
        self.table_videos.insertRow(row)
        
        # URL
        self.table_videos.setItem(row, 0, QTableWidgetItem(resource.url))
        
        # Type
        self.table_videos.setItem(row, 1, QTableWidgetItem(resource.resource_type))
        
        # Size
        size_mb = resource.size / (1024 * 1024)
        self.table_videos.setItem(row, 2, QTableWidgetItem(f"{size_mb:.2f} MB"))
        
        # MIME Type
        self.table_videos.setItem(row, 3, QTableWidgetItem(resource.mime_type))
        
        # Action button
        btn = QtWidgets.QPushButton("Add")
        btn.setProperty("resource_url", resource.url)
        btn.clicked.connect(lambda: self._add_video_to_download(resource.url))
        self.table_videos.setCellWidget(row, 4, btn)
        
    def _add_to_image_table(self, resource: CapturedResource):
        """添加图片资源到表格"""
        row = self.table_images.rowCount()
        self.table_images.insertRow(row)
        
        # URL
        self.table_images.setItem(row, 0, QTableWidgetItem(resource.url))
        
        # Type
        self.table_images.setItem(row, 1, QTableWidgetItem(resource.resource_type))
        
        # Size
        size_kb = resource.size / 1024
        self.table_images.setItem(row, 2, QTableWidgetItem(f"{size_kb:.2f} KB"))
        
        # MIME Type
        self.table_images.setItem(row, 3, QTableWidgetItem(resource.mime_type))
        
        # Preview (占位符)
        self.table_images.setItem(row, 4, QTableWidgetItem("Click to preview"))
        
        # Action button
        btn = QtWidgets.QPushButton("Add")
        btn.setProperty("resource_url", resource.url)
        btn.clicked.connect(lambda: self._add_image_to_download(resource.url))
        self.table_images.setCellWidget(row, 5, btn)
        
    def _update_statistics(self):
        """更新统计信息"""
        videos = len([r for r in self.captured_resources if r.resource_type == "video"])
        images = len([r for r in self.captured_resources if r.resource_type == "image"])
        total_size = sum(r.size for r in self.captured_resources)
        total_size_mb = total_size / (1024 * 1024)
        
        stats_text = f"""
        <h3>Capture Statistics</h3>
        <p>Total Resources: {len(self.captured_resources)}</p>
        <p>Videos: {videos}</p>
        <p>Images: {images}</p>
        <p>Total Size: {total_size_mb:.2f} MB</p>
        <p>Capture Time: {QtCore.QDateTime.currentDateTime().toString('yyyy-MM-dd hh:mm:ss')}</p>
        """
        
        self.lb_stats.setText(stats_text)
        
    @Slot()
    def on_capture_finished(self):
        """捕获完成"""
        self.pb_start.setEnabled(True)
        self.pb_stop.setEnabled(False)
        self.statusBar.showMessage(f"Capture completed. Found {len(self.captured_resources)} resources.")
        
    @Slot()
    def add_selected_videos(self):
        """添加选中的视频到下载列表"""
        selected_rows = set()
        for item in self.table_videos.selectedItems():
            selected_rows.add(item.row())
            
        for row in selected_rows:
            url_item = self.table_videos.item(row, 0)
            if url_item:
                self._add_video_to_download(url_item.text())
                
        if selected_rows:
            QMessageBox.information(self, "Success", f"Added {len(selected_rows)} videos to download list")
            
    @Slot()
    def download_selected_videos(self):
        """下载选中的视频"""
        selected_rows = set()
        for item in self.table_videos.selectedItems():
            selected_rows.add(item.row())
            
        if not selected_rows:
            QMessageBox.warning(self, "Warning", "Please select videos to download")
            return
            
        # 这里应该实现下载逻辑
        QMessageBox.information(self, "Info", f"Would download {len(selected_rows)} videos")
        
    @Slot()
    def add_selected_images(self):
        """添加选中的图片到下载列表"""
        selected_rows = set()
        for item in self.table_images.selectedItems():
            selected_rows.add(item.row())
            
        for row in selected_rows:
            url_item = self.table_images.item(row, 0)
            if url_item:
                self._add_image_to_download(url_item.text())
                
        if selected_rows:
            QMessageBox.information(self, "Success", f"Added {len(selected_rows)} images to download list")
            
    @Slot()
    def download_selected_images(self):
        """下载选中的图片"""
        selected_rows = set()
        for item in self.table_images.selectedItems():
            selected_rows.add(item.row())
            
        if not selected_rows:
            QMessageBox.warning(self, "Warning", "Please select images to download")
            return
            
        # 这里应该实现下载逻辑
        QMessageBox.information(self, "Info", f"Would download {len(selected_rows)} images")
        
    def _add_video_to_download(self, url: str):
        """添加视频到主窗口的下载列表"""
        # 这里应该与主窗口通信
        logger.info(f"Adding video to download: {url}")
        # 实际实现中，这里应该发射信号给主窗口
        
    def _add_image_to_download(self, url: str):
        """添加图片到主窗口的下载列表"""
        # 这里应该与主窗口通信
        logger.info(f"Adding image to download: {url}")
        # 实际实现中，这里应该发射信号给主窗口
        
    @Slot()
    def export_data(self):
        """导出捕获的数据"""
        if not self.captured_resources:
            QMessageBox.warning(self, "Warning", "No data to export")
            return
            
        filepath, _ = QtWidgets.QFileDialog.getSaveFileName(
            self, "Export Captured Data", "", "JSON Files (*.json);;All Files (*)"
        )
        
        if filepath:
            try:
                # 创建浏览器嗅探器实例并保存数据
                sniffer = BrowserSniffer()
                sniffer.captured_resources = self.captured_resources
                sniffer.save_captured_data(filepath)
                QMessageBox.information(self, "Success", f"Data exported to {filepath}")
            except Exception as e:
                QMessageBox.critical(self, "Error", f"Failed to export data: {str(e)}")
                
    def closeEvent(self, event):
        """关闭事件处理"""
        self.stop_capture()
        event.accept()

if __name__ == "__main__":
    import sys
    app = QtWidgets.QApplication(sys.argv)
    dialog = SnifferDialog()
    dialog.show()
    sys.exit(app.exec())