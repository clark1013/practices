//
//  PalleteEditor.swift
//  EmojiArt
//
//  Created by Wen Jiazhi on 2023/8/14.
//

import SwiftUI

struct PaletteEditor: View {
    @Binding var palette: Palette

    var body: some View {
        Form {
            nameSection
            addEmojiSection
            removeEmojiSection
        }
        .frame(minWidth: 300, minHeight: 400)
    }
    
    var nameSection: some View {
        Section("Name") {
            TextField("Name", text: $palette.name)
        }
    }
    
    @State var emojisToAdd: String = ""

    var addEmojiSection: some View {
        Section("Add Emojis") {
            TextField("", text: $emojisToAdd)
                .onChange(of: emojisToAdd) { emojis in
                    addEmojis(emojis: emojis)
                }
        }
    }
    
    private func addEmojis(emojis: String) {
        withAnimation {
            palette.emojis = (emojis + palette.emojis)
                .filter({ $0.isEmoji })
                .squeezed
        }
    }
    
    var removeEmojiSection: some View {
        Section("Remove Emojis") {
            let emojis = palette.emojis.squeezed.map({ String($0) })
            LazyVGrid(columns: [GridItem(.adaptive(minimum: 40))]) {
                ForEach(emojis, id: \.self) {emoji in
                    Text(emoji).onTapGesture {
//                        palette.emojis = palette.emojis.filter( {String($0) != emoji} )
                        palette.emojis.removeAll(where: {String($0) != emoji} )
                    }
                }
            }
        }
    }
}

struct PalleteEditor_Previews: PreviewProvider {
    static var previews: some View {
        let palette = PaletteStore(name: "Test").getPalette(at: 2)
        PaletteEditor(palette: .constant(palette))
            .previewLayout(.fixed(width: 300, height: 400))
    }
}
