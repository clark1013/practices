//
//  ContentView.swift
//  EmojiArt
//
//  Created by Wen Jiazhi on 2023/7/30.
//

import SwiftUI

struct EmojiArtDocumentView: View {
    @ObservedObject var document: EmojiArtDocument
    
    let defaultEmojiFontSize: CGFloat = 40

    var body: some View {
        VStack(spacing: 0) {
            documentBody
            palette
        }
    }
    
    var documentBody: some View {
        GeometryReader {geometry in
            ZStack {
                Color.white.overlay(
                    OptionalImage(uiImage: document.backgroundImage)
                        .scaleEffect(zoomScale)
                        .position(convertFromEmojiCoordinates(location: (0,0), geometry: geometry))
                )
                .gesture(doubleTapToZoom(size: geometry.size))
                if document.backgroundImageFetchStatus == .fetching {
                    ProgressView().scaleEffect(2)
                } else {
                    ForEach(document.emojis) {emoji in
                        Text(emoji.text)
                            .font(.system(size: fontSize(for: emoji)))
                            .position(position(for: emoji, geometry: geometry))
                            .scaleEffect(zoomScale)
                    }
                }
            }
            .clipped()
            .onDrop(of: [.plainText, .url, .image], isTargeted: nil) { providers, location in
                return drop(providers: providers, at: location, geometry: geometry)
            }
            .gesture(zoomGesture().simultaneously(with: panGesture()))
        }
    }
    
    private func drop(providers: [NSItemProvider], at location: CGPoint, geometry: GeometryProxy) -> Bool {
        var found = providers.loadObjects(ofType: URL.self) { url in
            document.setBackground(.url(url.imageURL))
        }
        if !found {
            found = providers.loadObjects(ofType: UIImage.self) { image in
                if let imageData = image.jpegData(compressionQuality: 1.0) {
                    document.setBackground(.imageData(imageData))
                }
            }
        }
        if !found {
            found = providers.loadObjects(ofType: String.self) { string in
                if let emoji = string.first, emoji.isEmoji {
                    document.addEmoji(
                        String(emoji),
                        location: convertToEmojiCoordinates(location, geometry: geometry),
                        size: defaultEmojiFontSize / zoomScale
                    )
                }
            }
        }
        return found
    }
    
    private func fontSize(for emoji: EmojiArtModel.Emoji) -> CGFloat {
        CGFloat(emoji.size)
    }
    
    private func position(for emoji: EmojiArtModel.Emoji, geometry: GeometryProxy) -> CGPoint {
        return convertFromEmojiCoordinates(location: (emoji.x, emoji.y), geometry: geometry)
    }
    
    private func convertToEmojiCoordinates(_ location: CGPoint, geometry: GeometryProxy) -> (Int, Int) {
        let center = geometry.frame(in: .local).center
        let location = CGPoint(
            x: (location.x - panOffset.width - center.x) / zoomScale,
            y: (location.y - panOffset.height - center.y) / zoomScale
        )
        return (Int(location.x), Int(location.y))
    }
    
    private func convertFromEmojiCoordinates(location: (x: Int, y: Int), geometry: GeometryProxy) -> CGPoint {
        let center = geometry.frame(in: .local).center
        return CGPoint(
            x: center.x + panOffset.width + CGFloat(location.x) * zoomScale,
            y: center.y + panOffset.height + CGFloat(location.y) * zoomScale
        )
    }
    
    @State private var steadyPanOffset: CGSize = CGSize.zero
    @GestureState private var gesturePanOffset: CGSize = CGSize.zero
    
    private var panOffset: CGSize {
        steadyPanOffset + gesturePanOffset
    }
    
    private func panGesture() -> some Gesture {
        DragGesture()
            .updating($gesturePanOffset) {currentPanState, gesturePanOffset, transaction in
                gesturePanOffset = currentPanState.translation
            }
            .onEnded{finalPanState in
                steadyPanOffset = (steadyPanOffset + finalPanState.translation) * zoomScale
            }
    }
    
    @State private var steadyZoomScale: CGFloat = 1
    @GestureState private var gestureZoomScale: CGFloat = 1
    
    private var zoomScale: CGFloat {
        steadyZoomScale * gestureZoomScale
    }

    private func zoomToFit(_ image: UIImage?, size: CGSize) {
        if let image = image, image.size.width > 0, image.size.height > 0, size.width > 0, size.height > 0 {
            let hZoom = size.width / image.size.width
            let vZoom = size.height / image.size.height
            steadyPanOffset = CGSize.zero
            steadyZoomScale = min(hZoom, vZoom)
        }
    }
    
    private func doubleTapToZoom(size: CGSize) -> some Gesture {
        TapGesture(count: 2)
            .onEnded {
                withAnimation {
                    zoomToFit(document.backgroundImage, size: size)
                }
            }
    }
    
    private func zoomGesture() -> some Gesture {
        MagnificationGesture()
            .updating($gestureZoomScale) {currentGestureScale, gestureZoomScale, transaction in
                gestureZoomScale = currentGestureScale
            }
            .onEnded {gestureScaleAtEnded in
                steadyZoomScale *= gestureScaleAtEnded
            }
    }
    
    var palette: some View {
        ScollingEmojisView(emojis: testEmojis)
            .font(.system(size: defaultEmojiFontSize))
    }
    
    var testEmojis = "🍋🍈🐗🐤🐦‍⬛🪯😃😍🤣"
}

struct ScollingEmojisView: View {
    var emojis: String

    var body: some View {
        ScrollView(.horizontal) {
            HStack {
                ForEach(emojis.map({String($0)}), id: \.self) { emoji in
                    Text(emoji)
                        .onDrag {NSItemProvider(object: emoji as NSString)}
                }
            }
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        EmojiArtDocumentView(document: EmojiArtDocument())
    }
}
