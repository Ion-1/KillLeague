import QtQml 2.15
import QtQuick 2.15
import QtQuick.Layouts 2.15
import QtQuick.Controls 2.15

Window {
    id: window
    width: 400
    height: 400
    title: qsTr("Kill League")
    visible: true

    ColumnLayout {
        anchors.fill: parent

        Button {
            text: qsTr("Murder League")
            onClicked: Backend.kill_league()
            Layout.fillWidth: true
            Layout.fillHeight: true
        }

        Button {
            text: qsTr("Kill Client")
            onClicked: Backend.kill_client()
            Layout.fillWidth: true
            Layout.fillHeight: true
        }

        Button {
            text: qsTr("Kill Riot Client")
            onClicked: Backend.kill_riot()
            Layout.fillWidth: true
            Layout.fillHeight: true
        }

        Button {
            text: qsTr("Kill Riot Services")
            onClicked: Backend.kill_service()
            Layout.fillWidth: true
            Layout.fillHeight: true
        }
    }
}