import sys
from os import path
from PySide6.QtWidgets import QApplication, QWidget, QLineEdit, QPushButton, QVBoxLayout, QMessageBox

def write_desktop_file(**kwargs):
    title = kwargs["title"]
    binary = kwargs["binary"]
    image = kwargs["image"]
    comment = kwargs["comment"]
    st = f"[Desktop entry]\nType=Application\nExec={binary}\nIcon={image}\nName={title}\nnComment={comment}"
    with open(path.join(path.expanduser("~"), ".local", "share", "applications", f"{title}.desktop"), "w") as f:
        f.write(st)
        f.close()
        QMessageBox(None, "Done", "Added .desktop file to the applications menu.").show()

class Window(QWidget):
    def write(self) -> None:
        title = self.title.text()
        binary = self.binary.text()
        image = self.image.text()
        comment = self.comment.text()
        write_desktop_file(title=title, binary=binary, image=image, comment=comment)
    def __init__(self) -> None:
        super().__init__()
        self.setWindowTitle(".desktop generator")
        self.layout = QVBoxLayout(self)

        self.title = QLineEdit()
        self.title.setPlaceholderText("Title")

        self.binary = QLineEdit()
        self.binary.setPlaceholderText("Binary")

        self.image = QLineEdit()
        self.image.setPlaceholderText("Image")

        self.comment = QLineEdit()
        self.comment.setPlaceholderText("Comment")

        self.btn = QPushButton("Add")
        self.btn.clicked.connect(self.write)

        self.layout.addWidget(self.title)
        self.layout.addWidget(self.binary)
        self.layout.addWidget(self.image)
        self.layout.addWidget(self.comment)
        self.layout.addWidget(self.btn)
        self.layout.addStretch()

def main():
    app = QApplication()
    win = Window()
    win.show()
    sys.exit(app.exec())


if __name__ == '__main__':
    main()