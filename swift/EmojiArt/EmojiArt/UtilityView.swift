//
//  UtilityView.swift
//  EmojiArt
//
//  Created by Wen Jiazhi on 2023/8/2.
//

import SwiftUI

struct OptionalImage: View {
    var uiImage: UIImage?
    
    var body: some View {
        if uiImage != nil {
            Image(uiImage: uiImage!)
        }
    }
}
