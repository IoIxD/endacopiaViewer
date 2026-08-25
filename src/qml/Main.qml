import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtMultimedia

import EndacopiaModTool

ApplicationWindow {
    visible: true
    title: qsTr("EndacopiaModTool")
    width: 800
    height: 600
    minimumWidth: 800
    minimumHeight: 600

    Backend {
        id: backend

        onUpdateFilenames: items => {
            for (let i = 0; i < items.length; i++) {
                listmodel.append({
                    name: items[i]
                });
            }
        }
        onShowPlayerMenu: {
            audiocontrols.visible = true;
        }
        onImageDisplay: {
            imgview.visible = true;
            imgview.width = backend.width;
            imgview.height = backend.height;
            imgview.source = backend.image_data_url;
        }
    }

    RowLayout {
        id: row
        anchors.fill: parent
        spacing: 10

        ScrollView {
            Layout.minimumWidth: 240
            Layout.maximumWidth: 240
            Layout.fillWidth: true
            Layout.fillHeight: true
            ScrollBar.vertical.policy: ScrollBar.AlwaysOn

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
                        imgview.visible = false;
                        backend.do_file_action(text);
                    }
                }
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true

            RowLayout {
                id: audiocontrols
                Layout.alignment: Qt.Alignment.AlignCenter
                visible: false

                Layout.margins: 8
                spacing: 8
                Button {
                    id: playButton
                    icon.name: "media-playback-start"
                    visible: true
                    onClicked: backend.play_sound()
                }
                Button {
                    id: pauseButton
                    icon.name: "media-playback-pause"
                    visible: false
                    onClicked: backend.play_sound()
                }
                Button {
                    id: stopButton
                    icon.name: "media-playback-stop"
                    onClicked: backend.stop_sound()
                }

                Text {
                    id: playbackTimer
                    text: "0:00 / 0:00"
                    color: parent.palette.text
                }
            }

            Image {
                id: imgview
                visible: false
                source: backend.image_data_url
                fillMode: Image.Stretch
            }
        }
    }
    Timer {
        interval: 1
        running: true
        repeat: true
        onTriggered: {
            if (backend.sound_playing()) {
                playButton.visible = false;
                pauseButton.visible = true;
            } else {
                playButton.visible = true;
                pauseButton.visible = false;
            }
            playbackTimer.text = backend.sound_pos() + " / " + backend.sound_len();
        }
    }

    Component.onCompleted: {
        backend.setup();
        audiocontrols.visible = false;
    }
}
