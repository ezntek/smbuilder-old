// Copyright 2023 Eason Qin <eason@ezntek.com>.
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::common::Versions;

pub enum RenderAPI {
    GL,
    GLLegacy,
    D3D11,
    D3D12,
}

pub enum Sm64exCoopRenderAPI {
    GL,
    GLLegacy,
    D3D11,
    D3D12,
    Dummy,
}

pub enum Render96WindowAPI {
    SDL2,
    DXGI,
}

pub enum Sm64exWindowAPI {
    SDL1,
    SDL2,
    DXGI,
}

pub enum Sm64exCoopWindowAPI {
    SDL1,
    SDL2,
    DXGI,
}

pub enum Sm64exAudioAPI {
    SDL1,
    SDL2,
}

pub enum Sm64exCoopAudioAPI {
    SDL1,
    SDL2,
    Dummy,
}

pub enum Sm64exControllerAPI {
    SDL1,
    SDL2,
}

pub enum Render96exMakeopts {
    Debug,
    Compare,
    NonMatching,
    TargetRpi,
    TargetWeb,
    OSXBuild,
    TargetArch(String),
    TargetBits(u8),
    BetterCamera,
    NoDrawingDistance,
    TextureFix,
    ExtendedOptionsMenu,
    TextSaves,
    ExternalData,
    DiscordRichPresence,
    RenderApi(RenderAPI),
    WindowApi(Render96WindowAPI),
    ModernRendering,
    WindowsBuild,
}

pub enum Sm64exMakeopts {
    Debug,
    Compare,
    NonMatching,
    TargetRpi,
    TargetWeb,
    OSXBuild,
    NoPie,
    TargetArch(String),
    TargetBits(u8),
    BetterCamera,
    NoDrawingDistance,
    TextureFix,
    ExtendedOptionsMenu,
    TextSaves,
    ExternalData,
    DiscordRichPresence,
    RenderApi(RenderAPI),
    WindowApi(Sm64exWindowAPI),
    AudioApi(Sm64exAudioAPI),
    ControllerApi(Sm64exControllerAPI),
    ModernRendering,
    WindowsBuild,
}

pub enum Sm64exCoopMakeopts {
    Debug,
    Development,
    TargetN64, 
    TargetRpi,
    TargetWeb,
    OSXBuild,
    TargetArch(String),
    TargetBits(u8),
    ImmediateLoad,
    BetterCamera,
    NoDrawingDistance,
    TextureFix,
    ExtendedOptionsMenu,
    TextSaves,
    ExternalData,
    DiscordRichPresence,
    DiscordSdk,
    DockerBuild,
    OptLevel(i8),
    DebugInfoLevel(u8),
    Profile,
    Headless,
    Icon,
    UseAppMacOS,
    RenderApi(Sm64exCoopRenderAPI),
    WindowApi(Sm64exCoopWindowAPI),
    AudioApi(Sm64exCoopAudioAPI),
    ModernRendering,
    WindowsBuild,
}

pub trait Makeopts {}
impl Makeopts for Sm64exCoopMakeopts {}
impl Makeopts for Sm64exMakeopts {}
impl Makeopts for Render96exMakeopts {}

fn get_default_makeopts(for_version: Versions) -> Vec<impl Makeopts> {
    match for_version {
        Versions::Render96ex => Vec::from([
            Render96exMakeopts::Compare,
            Render96exMakeopts::NonMatching,
            Render96exMakeopts::TargetArch(String::from("native")),
            Render96exMakeopts::TargetBits(0),
            Render96exMakeopts::ExtendedOptionsMenu,
            Render96exMakeopts::RenderApi(RenderAPI::GL),
            Render96exMakeopts::WindowApi(Render96WindowAPI::SDL2),
        ]),
        Versions::Sm64ex => Vec::from([
            Sm64exMakeopts::Compare,
            Sm64exMakeopts::NonMatching,
            Sm64exMakeopts::NoPie,
            Sm64exMakeopts::TargetArch(String::from("native")),
            Sm64exMakeopts::TargetBits(0),
            Sm64exMakeopts::ExtendedOptionsMenu,
            Sm64exMakeopts::RenderApi(RenderAPI::GL),
            Sm64exMakeopts::WindowApi(Sm64exWindowAPI::SDL2),
            Sm64exMakeopts::AudioApi(Sm64exAudioAPI::SDL2),
            Sm64exMakeopts::ControllerApi(Sm64exControllerAPI::SDL2),
            
        ]),
        Versions::Sm64exCoop => Vec::from([

        ]),
    }
}