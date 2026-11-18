<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, ref } from 'vue'

import { type Locale, useI18n } from '@open-pencil/vue'

import { recoveryEnabled, setRecoveryEnabled } from '@/app/document/recovery/preferences'
import { setSnappingPreference } from '@/app/settings/preferences/apply'
import {
  appPreferences,
  updateWindowPreferences
} from '@/app/settings/preferences/store'
import { animationPreference } from '@/app/shell/motion'
import { useAppTheme } from '@/app/shell/theme'
import { isTauri } from '@/app/tauri/env'
import CredentialSettingsSection from '@/components/settings/credentials/CredentialSettingsSection.vue'
import AccentColorPicker from '@/components/settings/general/AccentColorPicker.vue'
import CollaborationNameSection from '@/components/settings/general/CollaborationNameSection.vue'
import RenderingSettingsSection from '@/components/settings/general/RenderingSettingsSection.vue'
import SettingsGroup from '@/components/settings/layout/SettingsGroup.vue'
import SettingsSection from '@/components/settings/layout/SettingsSection.vue'
import AppSelect from '@/components/ui/select/AppSelect.vue'
import AppSwitch from '@/components/ui/toggle/AppSwitch.vue'

const { availableLocales, locale, localeLabels, menu, recovery, setLocale, settings } = useI18n()

const { theme } = useAppTheme()
const selectUI = { trigger: 'w-full sm:w-52' }

const language = computed<Locale>({
  get: () => locale.value,
  set: setLocale
})

const languageOptions = availableLocales.map((value) => ({
  value,
  label: localeLabels[value]
}))

const preserveUnsavedWork = computed({
  get: () => recoveryEnabled.value,
  set: setRecoveryEnabled
})

const snapToGeometry = computed({
  get: () => appPreferences.value.editing.snapping.geometry,
  set: (enabled: boolean) => setSnappingPreference('geometry', enabled)
})

const snapToObjects = computed({
  get: () => appPreferences.value.editing.snapping.objects,
  set: (enabled: boolean) => setSnappingPreference('objects', enabled)
})

const snapToPixelGrid = computed({
  get: () => appPreferences.value.editing.snapping.pixelGrid,
  set: (enabled: boolean) => setSnappingPreference('pixelGrid', enabled)
})

const desktopAvailable = ref(false)
const autostartAvailable = ref(false)
const autostartEnabled = ref(false)

onMounted(async () => {
  if (!isTauri()) return
  desktopAvailable.value = true
  autostartAvailable.value = await invoke<boolean>('autostart_supported').catch(() => false)
  if (autostartAvailable.value) {
    autostartEnabled.value = await invoke<boolean>('autostart_enabled').catch(() => false)
  }
})

const startWithWindows = computed({
  get: () => autostartEnabled.value,
  set: (enabled: boolean) => {
    autostartEnabled.value = enabled
    void invoke('set_autostart', { enabled }).catch(() => {
      autostartEnabled.value = !enabled
    })
  }
})

const closeToTray = computed({
  get: () => appPreferences.value.window.closeToTray,
  set: (enabled: boolean) => updateWindowPreferences({ closeToTray: enabled })
})

const minimizeToTray = computed({
  get: () => appPreferences.value.window.minimizeToTray,
  set: (enabled: boolean) => updateWindowPreferences({ minimizeToTray: enabled })
})
</script>

<template>
  <section class="flex flex-col gap-6" data-test-id="settings-general-panel">
    <SettingsSection>
      <template #title>{{ menu.language }}</template>
      <template #description>{{ settings.languageDescription }}</template>
      <SettingsGroup>
        <label class="flex items-center justify-between gap-4 px-3 py-2.5">
          <span class="text-xs text-surface">{{ menu.language }}</span>
          <AppSelect
            v-model="language"
            :label="menu.language"
            :options="languageOptions"
            :ui="selectUI"
            data-test-id="settings-language"
          />
        </label>
      </SettingsGroup>
    </SettingsSection>

    <SettingsSection>
      <template #title>{{ settings.appearance }}</template>
      <SettingsGroup>
        <label class="flex items-center justify-between gap-4 px-3 py-2.5">
          <span class="text-xs text-surface">{{ menu.theme }}</span>
          <AppSelect
            v-model="theme"
            :label="menu.theme"
            :options="[
              { value: 'auto', label: menu.themeAuto },
              { value: 'light', label: menu.themeLight },
              { value: 'dark', label: menu.themeDark }
            ]"
            :ui="selectUI"
          />
        </label>
        <div class="flex flex-wrap items-center justify-between gap-x-4 gap-y-2 px-3 py-2.5">
          <span class="text-xs text-surface">{{ settings.accentColor }}</span>
          <AccentColorPicker />
        </div>
        <label class="flex items-center justify-between gap-4 px-3 py-2.5">
          <span class="text-xs text-surface">{{ settings.animations }}</span>
          <AppSelect
            v-model="animationPreference"
            :label="settings.animations"
            :options="[
              { value: 'system', label: settings.animationsSystem },
              { value: 'off', label: settings.animationsOff }
            ]"
            :ui="selectUI"
          />
        </label>
      </SettingsGroup>
    </SettingsSection>

    <CollaborationNameSection />

    <SettingsSection>
      <template #title>{{ recovery.settingsTitle }}</template>

      <SettingsGroup>
        <label class="flex items-center justify-between gap-4 px-3 py-2.5">
          <span>
            <span class="block text-xs text-surface">{{ recovery.preserveUnsavedWork }}</span>
            <span class="block text-[10px] text-muted">{{
              recovery.preserveUnsavedWorkDescription
            }}</span>
          </span>
          <AppSwitch
            v-model="preserveUnsavedWork"
            :label="recovery.preserveUnsavedWork"
            data-test-id="settings-recovery-enabled"
          />
        </label>
      </SettingsGroup>
    </SettingsSection>

    <SettingsSection v-if="desktopAvailable">
      <template #title>{{ settings.desktop }}</template>
      <template #description>{{ settings.desktopDescription }}</template>

      <SettingsGroup>
        <label
          v-if="autostartAvailable"
          class="flex items-center justify-between gap-4 px-3 py-2.5"
        >
          <span>
            <span class="block text-xs text-surface">{{ settings.startWithWindows }}</span>
            <span class="block text-[10px] text-muted">{{
              settings.startWithWindowsDescription
            }}</span>
          </span>
          <AppSwitch
            v-model="startWithWindows"
            :label="settings.startWithWindows"
            data-test-id="settings-start-with-windows"
          />
        </label>
        <label class="flex items-center justify-between gap-4 px-3 py-2.5">
          <span>
            <span class="block text-xs text-surface">{{ settings.closeToTray }}</span>
            <span class="block text-[10px] text-muted">{{ settings.closeToTrayDescription }}</span>
          </span>
          <AppSwitch
            v-model="closeToTray"
            :label="settings.closeToTray"
            data-test-id="settings-close-to-tray"
          />
        </label>
        <label class="flex items-center justify-between gap-4 px-3 py-2.5">
          <span>
            <span class="block text-xs text-surface">{{ settings.minimizeToTray }}</span>
            <span class="block text-[10px] text-muted">{{
              settings.minimizeToTrayDescription
            }}</span>
          </span>
          <AppSwitch
            v-model="minimizeToTray"
            :label="settings.minimizeToTray"
            data-test-id="settings-minimize-to-tray"
          />
        </label>
      </SettingsGroup>
    </SettingsSection>

    <SettingsSection>
      <template #title>{{ settings.editing }}</template>
      <template #description>{{ settings.snappingDescription }}</template>

      <SettingsGroup>
        <label class="flex items-center justify-between gap-4 px-3 py-2.5">
          <span>
            <span class="block text-xs text-surface">{{ settings.snapToGeometry }}</span>
            <span class="block text-[10px] text-muted">{{
              settings.snapToGeometryDescription
            }}</span>
          </span>
          <AppSwitch
            v-model="snapToGeometry"
            :label="settings.snapToGeometry"
            data-test-id="settings-snap-geometry"
          />
        </label>
        <label class="flex items-center justify-between gap-4 px-3 py-2.5">
          <span>
            <span class="block text-xs text-surface">{{ settings.snapToObjects }}</span>
            <span class="block text-[10px] text-muted">{{
              settings.snapToObjectsDescription
            }}</span>
          </span>
          <AppSwitch
            v-model="snapToObjects"
            :label="settings.snapToObjects"
            data-test-id="settings-snap-objects"
          />
        </label>
        <label class="flex items-center justify-between gap-4 px-3 py-2.5">
          <span>
            <span class="block text-xs text-surface">{{ settings.snapToPixelGrid }}</span>
            <span class="block text-[10px] text-muted">{{
              settings.snapToPixelGridDescription
            }}</span>
          </span>
          <AppSwitch
            v-model="snapToPixelGrid"
            :label="settings.snapToPixelGrid"
            data-test-id="settings-snap-pixel-grid"
          />
        </label>
      </SettingsGroup>

      <p class="text-[11px] text-muted">{{ settings.temporaryDisableSnappingHint }}</p>
    </SettingsSection>

    <CredentialSettingsSection />
    <RenderingSettingsSection />
  </section>
</template>
