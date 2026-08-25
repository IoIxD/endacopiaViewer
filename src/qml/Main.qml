import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtMultimedia

import endacopiaViewer

ApplicationWindow {
    visible: true
    title: qsTr("EndacopiaModTool")
    width: 640
    height: 480

    Backend {
        id: backend

        onFilenamesChanged: items => {
            for (let i = 0; i < items.length; i++) {
                listmodel.append({
                    name: items[i]
                });
            }
        }
    }

    RowLayout {
        id: row
        anchors.fill: parent
        spacing: 10

        ListView {
            id: listview
            Layout.alignment: Qt.Alignment.AlignLeft
            Layout.fillWidth: true
            Layout.fillHeight: true
            currentIndex: -1

            model: ListModel {
                id: listmodel
            }

            delegate: ItemDelegate {
                width: ListView.view.width
                text: modelData
                highlighted: ListView.isCurrentItem
                onClicked: {
                    listview.currentIndex = index;
                    let text = listview.itemAtIndex(index).text.toString();
                    audiocontrols.visible = false;
                    if (text.includes(".ogg")) {
                        backend.sound_selected(text);
                        audiocontrols.visible = true;
                    }
                }
            }
        }

        RowLayout {
            id: audiocontrols
            Layout.alignment: Qt.Alignment.AlignRight
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.margins: 8
            spacing: 8
            Button {
                text: "Play"
                onClicked: player.play()
            }
            Button {
                text: "Pause"
                onClicked: player.pause()
            }
            Button {
                text: "Stop"
                onClicked: player.stop()
            }
        }
    }

    Component.onCompleted: {
        backend.setup();
        audiocontrols.visible = false;
    }
}
