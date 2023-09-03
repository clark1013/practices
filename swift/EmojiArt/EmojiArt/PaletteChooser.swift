//
//  PaletteChooser.swift
//  EmojiArt
//
//  Created by Wen Jiazhi on 2023/8/11.
//

import SwiftUI

struct PaletteChooser: View {
    var fontSize: CGFloat = 40
    @EnvironmentObject var store: PaletteStore
    @State private var choosenPaletteIndex: Int = 0
    @State private var paletteToEdit: Palette?
    
    var emojiFont: Font { .system(size: fontSize)}

    var body: some View {
        let palette = store.getPalette(at: choosenPaletteIndex)
        HStack {
            paletteChangeButton
            body(for: palette)
        }
    }
    
    var paletteChangeButton: some View {
        Button {
            choosenPaletteIndex = (choosenPaletteIndex + 1) % store.palettes.count
        } label: {
            Image(systemName: "paintpalette")
        }
        .font(emojiFont)
        .contextMenu{ contextMenu }
    }
    
    @ViewBuilder
    var contextMenu: some View {
        AnimatedActionButton(title: "Edit", systemImage: "pencil") {
            paletteToEdit = store.palettes[choosenPaletteIndex]
        }
        AnimatedActionButton(title: "New", systemImage: "plus") {
            store.insertPalette(named: "New", emojis: "", at: choosenPaletteIndex)
            paletteToEdit = store.palettes[choosenPaletteIndex]
        }
        AnimatedActionButton(title: "Delete", systemImage: "minus.circle") {
            choosenPaletteIndex = store.removePalette(at: choosenPaletteIndex)
        }
        gotoMenu
    }
    
    var gotoMenu: some View {
        Menu {
            ForEach( store.palettes ) { palette in
                AnimatedActionButton(title: palette.name) {
                    if let index = store.palettes.firstIndex(where: { $0.id == palette.id }) {
                        choosenPaletteIndex = index
                    }
                }
            }
        } label: {
            Label("Go To", systemImage: "text.insert")
        }
    }
    
    func body(for palette: Palette) -> some View {
        HStack{
            Text(palette.name)
            ScollingEmojisView(emojis: palette.emojis)
                .font(emojiFont)
        }
        .popover(item: $paletteToEdit) { palette in
            PaletteEditor(palette: $store.palettes.first(where: { $0.id == palette.id })!)
                .presentationCompactAdaptation((.popover))
        }
    }
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

struct PaletteChooser_Previews: PreviewProvider {
    static var previews: some View {
        PaletteChooser()
    }
}
