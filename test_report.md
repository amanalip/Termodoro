# Termodoro Test Report & Quality Assurance Audit

**Execution Date:** August 21, 2026  
**Result:** **259 / 259 Tests Passed (100% Success Rate)**  
**Duration:** ~0.61s  

---

## 1. Executive Summary

A comprehensive, rigorous quality assurance (QA) overhaul across all layers of the **Termodoro** application was conducted.

Test coverage now spans all **9 core modules plus the rigorous integration E2E suite** with **259 unit, integration, and end-to-end tests** (expanded across all iterations from 7 -> 74 -> 91 -> 93 -> 111 -> 120 -> 126 -> 137 -> 151 -> 154 -> 192 -> 199 -> 257 -> 259). The test suite verifies every button, key combination, modal state, unhandled modifier, phase transition combinations with auto-start flags, interval boundaries (1 to 24), Unicode & emoji task handling, empty & partial JSON storage error recovery, zero telemetry and local-only filesystem privacy isolation invariants, synthesized audio signal integrity, audio tail decay click prevention, RIFF WAV header data offsets, multi-tab terminal rendering stress tests, extreme viewport geometries, 366-day leap year streak calculations, 18-theme WCAG luminance contrast formulas, full persistence-across-restart workflows, and 1,000-iteration random key input chaos fuzzing.

---

## 2. Test Suite Breakdown by Module

| Module | Test File | Test Cases | Category | Status |
| :--- | :--- | :---: | :---: | :---: |
| **Application State & End-to-End** | [`src/app.rs`](src/app.rs) | 41 | Key Matrix, Chaos Fuzzing, Auto-start, Direct Tabs & Reassignment | PASS |
| **Productivity Analytics & Streaks** | [`src/stats.rs`](src/stats.rs) | 29 | 366-Day Leap Year, Multi-Streak, Deduplication & Histograms | PASS |
| **Timer Engine** | [`src/timer.rs`](src/timer.rs) | 29 | 24-Cycle State Machine, Sub-Second Ticks & Formats | PASS |
| **Task Management** | [`src/tasks.rs`](src/tasks.rs) | 32 | UUIDs, Unicode, Boundaries, Filter Clamps & Target Rebinding | PASS |
| **Audio Engine & Chimes** | [`src/audio.rs`](src/audio.rs) | 20 | Acoustic QA, WAV Signals, Headroom & DAC Click Prevention | PASS |
| **Terminal UI Rendering** | [`src/ui/mod.rs`](src/ui/mod.rs) | 18 | Buffer Content, Extreme Geometries (350x120), Filter Views & 24 Dots | PASS |
| **Persistence & File Storage** | [`src/storage.rs`](src/storage.rs) | 22 | Zero Telemetry, Privacy Invariants, Atomic .tmp Cleanups & Recovery | PASS |
| **Themes & Color Palettes** | [`src/theme.rs`](src/theme.rs) | 16 | 18 Palettes, Contrast, RGB Bounds & Serde | PASS |
| **Configuration & Preferences** | [`src/config.rs`](src/config.rs) | 8 | Serde, Extreme Values, Boolean Flags & Defaults | PASS |
| **ASCII Big Digits Graphic UI** | [`src/ui/digits.rs`](src/ui/digits.rs) | 5 | Glyphs & Block Typography Bounds | PASS |
| **Rigorous Integration E2E** | [`tests/e2e_rigorous.rs`](tests/e2e_rigorous.rs) | 39 | Persistence-Across-Restart, Relaunch State, Multi-Cycle Workflows | PASS |
| **Total** | | **259** | **ALL PASSED** | **100%** |

---

## 3. Comprehensive Verification Matrix

| Test Category | Invariant Verified | Passing Proof |
| :--- | :--- | :---: |
| **Audio Waveforms** | 16-bit PCM RIFF Header, 44.1kHz, Zero Clipping, Exponential Decay | 20 / 20 PASS |
| **Timer FSM** | 24-Cycle Sub-second Decrement, Long Break Trigger, Skip-No-Credit Semantics, Phase Transitions | 29 / 29 PASS |
| **Task Operations** | UUID V4 Collision Proof, Unicode Cell-Width, Selection Clamping | 32 / 32 PASS |
| **Habit Streaks** | Leap Year Continuity, Month/Year Bridges, Zero Break Inflation | 29 / 29 PASS |
| **Air-Gapped Privacy** | Atomic .tmp File Swap, Zero Network/Telemetry Schema Keys | 22 / 22 PASS |
| **Theme Luminance** | WCAG 2.1 AA Contrast Ratio ($R \ge 4.5:1$) across all 18 Themes | 16 / 16 PASS |
| **TUI Geometries** | Viewport Rendering from $20\times 10$ to $350\times 120$ Ultra-Wide | 23 / 23 PASS |
| **Application E2E** | 1,000-Step Chaos Fuzzing, Modal Overlays, Toast Expirations | 41 / 41 PASS |
| **Configuration** | Serde Roundtrips, Flag Permutations, Clamping Compatibility | 8 / 8 PASS |
| **Integration E2E Suite** | Temp-Dir Persistence Roundtrips, Relaunch Restoration, Multi-Cycle Runs | 39 / 39 PASS |

---

## 4. Complete Certified List of All 259 Test Cases

1. `app::tests::test_all_eighteen_themes_cycle_and_persistence_e2e`
2. `app::tests::test_all_eighteen_themes_full_ui_render_all_tabs_e2e`
3. `app::tests::test_all_settings_rows_min_max_clamping_exhaustive`
4. `app::tests::test_app_direct_tab_numeric_navigation_integration`
5. `app::tests::test_app_help_modal_all_dismiss_keys`
6. `app::tests::test_app_quit_command_handling`
7. `app::tests::test_app_restart_and_state_recovery_e2e`
8. `app::tests::test_app_settings_navigation_bounds_with_all_key_variants`
9. `app::tests::test_app_settings_toggle_all_boolean_rows`
10. `app::tests::test_app_status_message_overwrite_and_expiry`
11. `app::tests::test_app_task_modal_rapid_editing_backspace_and_navigation`
12. `app::tests::test_app_task_target_binding_and_unbinding_e2e`
13. `app::tests::test_break_session_records_break_phase_duration`
14. `app::tests::test_completed_session_records_actual_duration_not_mutated_config`
15. `app::tests::test_completed_session_records_actual_duration_when_config_shrunk`
16. `app::tests::test_empty_tasks_key_interactions_graceful`
17. `app::tests::test_full_pomodoro_cycle_e2e`
18. `app::tests::test_fuzz_randomized_key_events_chaos_resilience`
19. `app::tests::test_global_keys_tab_navigation`
20. `app::tests::test_help_modal_workflow`
21. `app::tests::test_keypresses_do_not_decrement_timer`
22. `app::tests::test_long_break_interval_boundary_one_and_twenty_four`
23. `app::tests::test_modal_backspace_empty_and_focus_toggle_chain`
24. `app::tests::test_modal_enter_shows_trimmed_title_in_status_message`
25. `app::tests::test_modal_exclusive_key_handling`
26. `app::tests::test_notify_phase_completed_sound_and_notification_flags`
27. `app::tests::test_phase_transitions_with_auto_start_disabled_combinations`
28. `app::tests::test_rapid_filter_switching_and_index_clamping_chaos`
29. `app::tests::test_settings_adjustments_and_clamping`
30. `app::tests::test_settings_tab_vim_keys_and_live_timer_updates`
31. `app::tests::test_status_message_expiration_on_ticks`
32. `app::tests::test_subsecond_tick_accumulation_and_second_decrement`
33. `app::tests::test_tab_navigation_methods`
34. `app::tests::test_task_modal_key_interactions`
35. `app::tests::test_task_modal_validation_and_bounds`
36. `app::tests::test_task_reassignment_on_deletion_and_completion_chain`
37. `app::tests::test_tasks_tab_filter_keys_do_not_switch_tabs`
38. `app::tests::test_tasks_tab_key_interactions`
39. `app::tests::test_timer_keys_and_on_tick_flow`
40. `app::tests::test_twenty_four_cycle_app_e2e_workflow`
41. `app::tests::test_unhandled_keys_and_modifier_combinations`
42. `audio::tests::test_audio_break_chime_two_tone_structure_duration`
43. `audio::tests::test_audio_long_break_chime_three_tone_triad_duration`
44. `audio::tests::test_audio_mute_flag_concurrency`
45. `audio::tests::test_audio_mute_for_tests_flag`
46. `audio::tests::test_audio_sample_amplitudes_fade_out_smoothly`
47. `audio::tests::test_audio_sample_rate_conversion_and_duration_math`
48. `audio::tests::test_audio_work_chime_harmonic_components_variance`
49. `audio::tests::test_create_riff_wav_byte_level_alignment`
50. `audio::tests::test_create_riff_wav_custom_sample_rates`
51. `audio::tests::test_create_riff_wav_empty_samples`
52. `audio::tests::test_create_riff_wav_pcm16_header`
53. `audio::tests::test_generate_break_complete_chime`
54. `audio::tests::test_generate_chimes_finite_and_clean_samples`
55. `audio::tests::test_generate_long_break_chime`
56. `audio::tests::test_generate_work_complete_chime`
57. `audio::tests::test_note_boundaries_are_click_free`
58. `audio::tests::test_play_phase_sound_does_not_panic`
59. `audio::tests::test_wav_header_subchunk2_size_consistency`
60. `audio::tests::test_wav_sample_bounds_no_clipping_break_chimes`
61. `audio::tests::test_wav_sample_bounds_no_clipping_work_chime`
62. `config::tests::test_config_all_theme_variant_serialization`
63. `config::tests::test_config_boolean_flag_combinations`
64. `config::tests::test_config_custom_initialization_builder_pattern`
65. `config::tests::test_config_debug_formatting`
66. `config::tests::test_config_extreme_duration_values_serde`
67. `config::tests::test_config_mutation_and_cloned_equality`
68. `config::tests::test_config_serde_roundtrip`
69. `config::tests::test_default_config_values`
70. `stats::tests::test_break_sessions_do_not_count_towards_work_stats`
71. `stats::tests::test_break_sessions_ignored_by_distinct_dates`
72. `stats::tests::test_complex_multi_streak_history`
73. `stats::tests::test_consecutive_multi_day_streaks_and_longest`
74. `stats::tests::test_distribution_formatting_weekdays`
75. `stats::tests::test_distribution_variable_day_windows`
76. `stats::tests::test_empty_stats_streak`
77. `stats::tests::test_full_year_leap_year_streak_simulation`
78. `stats::tests::test_last_days_distribution`
79. `stats::tests::test_multi_day_streak_yesterday_continuation`
80. `stats::tests::test_out_of_order_session_timestamps`
81. `stats::tests::test_same_day_multiple_sessions_dedup`
82. `stats::tests::test_stats_distribution_sum_matches_total`
83. `stats::tests::test_stats_empty_distribution_window_zeroes`
84. `stats::tests::test_stats_history_default_and_metadata`
85. `stats::tests::test_stats_large_volume_aggregation`
86. `stats::tests::test_stats_longest_streak_persists_after_streak_broken`
87. `stats::tests::test_stats_recent_sessions_ordering_and_task_attribution`
88. `stats::tests::test_stats_recent_sessions_pagination_and_slice`
89. `stats::tests::test_stats_recording`
90. `stats::tests::test_stats_serde_roundtrip`
91. `stats::tests::test_stats_streak_calculation_single_day_session_history`
92. `stats::tests::test_stats_today_work_sessions_and_minutes_accuracy`
93. `stats::tests::test_stats_total_focus_hours_formatting`
94. `stats::tests::test_stats_very_large_focus_minutes_accumulation_and_average`
95. `stats::tests::test_streak_broken_two_days_ago`
96. `stats::tests::test_streak_calculation_across_month_and_year_boundaries`
97. `stats::tests::test_streak_with_intermittent_breaks_and_restarts`
98. `stats::tests::test_streak_yesterday_preserved`
99. `storage::tests::test_appdata_empty_object_yields_full_defaults`
100. `storage::tests::test_appdata_missing_config_section_loads_with_defaults`
101. `storage::tests::test_appdata_missing_stats_section_loads_with_defaults`
102. `storage::tests::test_appdata_missing_tasks_section_loads_with_defaults`
103. `storage::tests::test_appdata_roundtrip_serde`
104. `storage::tests::test_privacy_zero_telemetry_guarantees`
105. `storage::tests::test_storage_app_data_clone_and_equality`
106. `storage::tests::test_storage_atomic_tmp_file_renamed_after_save`
107. `storage::tests::test_storage_constructor_new_default_path_exists`
108. `storage::tests::test_storage_corrupt_file_is_quarantined_not_overwritten`
109. `storage::tests::test_storage_custom_deep_path_creation`
110. `storage::tests::test_storage_custom_path_accessor_and_default_fallback`
111. `storage::tests::test_storage_data_isolation_local_only`
112. `storage::tests::test_storage_empty_file_and_partial_json_fallback`
113. `storage::tests::test_storage_failed_save_leaves_previous_data_intact`
114. `storage::tests::test_storage_fallback_on_nonexistent_or_corrupt_file`
115. `storage::tests::test_storage_legacy_file_without_stats_keeps_tasks`
116. `storage::tests::test_storage_load_idempotence`
117. `storage::tests::test_storage_load_sanitizes_out_of_range_config`
118. `storage::tests::test_storage_save_and_load_roundtrip`
119. `storage::tests::test_storage_save_and_load_with_full_dataset`
120. `storage::tests::test_storage_schema_fields_contain_no_device_or_telemetry_keys`
121. `tasks::tests::test_empty_and_whitespace_title_rejected`
122. `tasks::tests::test_increment_active_spent_no_active_task`
123. `tasks::tests::test_increment_active_spent_skips_completed_active_task`
124. `tasks::tests::test_navigation_next_previous_wrapping`
125. `tasks::tests::test_remove_selected_and_active_reassignment`
126. `tasks::tests::test_set_selected_active`
127. `tasks::tests::test_set_selected_active_accepts_incomplete_task`
128. `tasks::tests::test_set_selected_active_rejects_completed_task`
129. `tasks::tests::test_task_active_reassignment_on_deletion`
130. `tasks::tests::test_task_creation_with_zero_estimated_defaults_to_one`
131. `tasks::tests::test_task_deletion_at_different_positions`
132. `tasks::tests::test_task_filter_default`
133. `tasks::tests::test_task_filtered_list_view_integrity`
134. `tasks::tests::test_task_filtering`
135. `tasks::tests::test_task_index_clamping_across_dynamic_filter_changes`
136. `tasks::tests::test_task_lifecycle`
137. `tasks::tests::test_task_manager_default_and_invalid_active_lookup`
138. `tasks::tests::test_task_multiline_and_emoji_stress`
139. `tasks::tests::test_task_navigation_empty_manager_safety`
140. `tasks::tests::test_task_operations_with_active_filters`
141. `tasks::tests::test_task_selected_index_bounds_wrapping`
142. `tasks::tests::test_task_title_whitespace_trimming_and_sanitization`
143. `tasks::tests::test_task_uuid_uniqueness_and_timestamps`
144. `tasks::tests::test_tasks_empty_navigation_and_actions`
145. `tasks::tests::test_tasks_filtered_zero_matches_safety`
146. `tasks::tests::test_tasks_large_volume_performance`
147. `tasks::tests::test_tasks_serde_roundtrip_skip_fields`
148. `tasks::tests::test_tasks_unicode_and_estimate_limits`
149. `tasks::tests::test_tasks_with_special_characters`
150. `tasks::tests::test_toggle_selected_active_task_reassignment`
151. `tasks::tests::test_toggle_that_empties_filtered_view_resets_cursor`
152. `tasks::tests::test_toggle_that_shrinks_filtered_view_clamps_cursor`
153. `theme::tests::test_all_theme_choices`
154. `theme::tests::test_deserialize_unknown_theme_falls_back_to_default`
155. `theme::tests::test_from_str_handles_accented_and_apostrophe_names`
156. `theme::tests::test_from_str_loose_formatting_tolerated`
157. `theme::tests::test_from_str_parses_every_display_name`
158. `theme::tests::test_from_str_parses_every_variant_name`
159. `theme::tests::test_theme_choice_clone_and_copy`
160. `theme::tests::test_theme_choice_serde_roundtrip_all`
161. `theme::tests::test_theme_choice_unknown_name_falls_back_to_default`
162. `theme::tests::test_theme_default_fallback_is_catppuccin_mocha`
163. `theme::tests::test_theme_from_choice_all_variants`
164. `theme::tests::test_theme_luminance_contrast_across_all_18_palettes`
165. `theme::tests::test_theme_names`
166. `theme::tests::test_theme_palette_index_cycling`
167. `theme::tests::test_theme_phase_colors_distinctness`
168. `theme::tests::test_theme_rgb_components_within_byte_bounds`
169. `timer::tests::test_auto_start_settings_on_transition`
170. `timer::tests::test_formatted_time`
171. `timer::tests::test_formatted_time_large_values`
172. `timer::tests::test_natural_completion_still_credits_after_skips`
173. `timer::tests::test_pause_and_reset`
174. `timer::tests::test_phase_advancement`
175. `timer::tests::test_phase_titles_and_emojis`
176. `timer::tests::test_progress_ratio`
177. `timer::tests::test_skip_from_work_never_credits_pomodoro_or_cycle`
178. `timer::tests::test_skip_to_next`
179. `timer::tests::test_target_duration_secs_all_phases`
180. `timer::tests::test_tick_when_paused_or_stopped_does_nothing`
181. `timer::tests::test_tick_when_running_and_completion_event`
182. `timer::tests::test_timer_exact_phase_transition_cycle_counting`
183. `timer::tests::test_timer_formatted_time_zero_and_single_digits`
184. `timer::tests::test_timer_initialization`
185. `timer::tests::test_timer_long_break_to_work_cycle_reset`
186. `timer::tests::test_timer_multiple_consecutive_skips`
187. `timer::tests::test_timer_phase_title_and_emoji_completeness`
188. `timer::tests::test_timer_progress_ratio_bounds_and_rounding`
189. `timer::tests::test_timer_rapid_status_flipping_under_tick_loop`
190. `timer::tests::test_timer_reset_across_all_phases`
191. `timer::tests::test_timer_serde_roundtrip`
192. `timer::tests::test_timer_status_transitions_and_predicates`
193. `timer::tests::test_timer_target_duration_all_phases_with_custom_config`
194. `timer::tests::test_timer_time_remaining_never_underflows_sub_second_ticks`
195. `timer::tests::test_timer_toggle_transitions`
196. `timer::tests::test_timer_zero_total_duration_progress_ratio`
197. `timer::tests::test_twenty_four_cycle_advancement_and_long_break_trigger`
198. `ui::digits::tests::test_big_digits_various_large_minutes_formatting`
199. `ui::digits::tests::test_char_pattern_all_valid_chars`
200. `ui::digits::tests::test_render_big_time_boundary_values`
201. `ui::digits::tests::test_render_big_time_structure`
202. `ui::digits::tests::test_render_big_time_various_values`
203. `ui::tests::test_buffer_cell_content_assertions_across_views`
204. `ui::tests::test_render_active_task_card_details_on_timer_view`
205. `ui::tests::test_render_all_color_themes`
206. `ui::tests::test_render_all_settings_rows_highlighted`
207. `ui::tests::test_render_all_tabs_without_panic`
208. `ui::tests::test_render_all_terminal_dimensions`
209. `ui::tests::test_render_all_three_task_filter_tabs`
210. `ui::tests::test_render_all_timer_phases_and_statuses`
211. `ui::tests::test_render_empty_views`
212. `ui::tests::test_render_extreme_content_and_dimensions`
213. `ui::tests::test_render_extreme_high_resolution_terminal`
214. `ui::tests::test_render_extreme_small_terminals`
215. `ui::tests::test_render_modals_and_status_message`
216. `ui::tests::test_render_task_modal_both_focus_states`
217. `ui::tests::test_render_twenty_four_cycle_dots_timer_view`
218. `ui::tests::test_render_varied_terminal_geometries_stress`
219. `ui::tests::test_task_modal_focus_switch_and_cancel_invariants`
220. `ui::tests::test_ui_render_with_status_message_banner_content`
221. `big_digits_three_digit_minutes_uniform_width`
222. `chaos_filter_modal_interleave_maintains_core_invariants`
223. `chaos_fuzz_mixed_events_with_periodic_persistence_invariants`
224. `completed_sessions_record_actual_elapsed_duration_even_when_config_mutated`
225. `completed_task_cannot_become_active_target_via_keys`
226. `corrupt_state_file_is_quarantined_with_bytes_intact`
227. `delete_on_empty_and_filtered_out_lists_reports_noop`
228. `deleting_active_task_mid_day_reassigns_credit_to_next`
229. `distribution_window_sums_match_recorded_sessions`
230. `every_theme_survives_save_reload_cycle`
231. `filter_matrix_add_toggle_delete_maintains_integrity`
232. `footer_swaps_status_banner_back_to_hints_after_expiry`
233. `full_workday_simulation_with_restart_between_sessions`
234. `hand_edited_theme_display_names_survive_reload`
235. `help_popup_renders_on_tiny_terminal`
236. `large_task_list_survives_restart_roundtrip`
237. `long_break_interval_exhaustive_one_to_twenty_four`
238. `modal_estimate_input_mashing_stays_in_bounds`
239. `pause_resume_across_many_toggles_never_drifts`
240. `render_every_tab_modal_geometry_matrix_with_content`
241. `repeated_saves_leave_no_tmp_litter_and_stay_parseable`
242. `reset_keeps_cycle_position_and_clears_countdown`
243. `save_reload_roundtrip_preserves_spent_counts_and_stats`
244. `save_reload_roundtrip_preserves_unicode_titles_and_target`
245. `scrolled_task_list_keeps_selection_visible`
246. `selection_wrapping_under_all_filters_never_panics`
247. `skip_credits_nothing_and_never_advances_cycle`
248. `storage_loads_file_missing_config_section`
249. `storage_loads_file_missing_tasks_section`
250. `storage_loads_legacy_file_missing_stats_section_and_keeps_tasks`
251. `streak_continuity_across_month_and_year_boundary`
252. `streak_ignores_future_dated_entries_gracefully`
253. `timer_state_is_not_persisted_across_restart`
254. `timer_view_renders_extreme_countdown_values`
255. `today_metrics_exclude_other_days`
256. `transient_ui_state_resets_on_restart`
257. `whitespace_only_title_rejected_then_real_title_accepted`
258. `work_session_credits_only_the_active_task_across_retargeting`
259. `zero_duration_direct_construction_never_panics_or_underflows`

---

## 5. Playwright Automated Cross-Device Web E2E Test Suite

**Test Script:** [`scripts/e2e-website-test.mjs`](scripts/e2e-website-test.mjs)  
**Execution Command:** `make test-e2e` or `node scripts/e2e-website-test.mjs`  
**Browser Engine:** Chromium Headless (Automated Device Profiles)  
**Pass Rate:** **41 / 41 Tests Passed (100% Success Rate)**  

### Test Viewport Matrix

| Category | Viewport Resolution | Target Devices Simulated | Test Status |
| :--- | :---: | :--- | :---: |
| **Desktop Large** | `1920 × 1080` | Ultra-wide & 4K desktop screens | **PASS** (100%) |
| **Desktop Standard** | `1280 × 800` | Standard laptops & MacBook Air/Pro | **PASS** (100%) |
| **Tablet Portrait** | `768 × 1024` | Apple iPad, Android Tablets | **PASS** (100%) |
| **Mobile Flagship** | `390 × 844` | iPhone 14/15/16, Google Pixel 7/8 | **PASS** (100%) |
| **Mobile Medium** | `375 × 667` | iPhone SE (2nd/3rd gen), Galaxy A series | **PASS** (100%) |
| **Mobile Small** | `320 × 568` | iPhone 5/SE (1st gen), Compact screens | **PASS** (100%) |

### Verified Functional Suites

1. **Suite 1: Responsive Layout & Zero Horizontal Overflow** (18/18 checks)  
   - Validated `scrollWidth <= clientWidth` across all 6 viewport profiles on `index.html`, `features.html`, and `faqs.html`.
2. **Suite 2: Mobile Navigation Drawer & Hamburger Interactions** (6/6 checks)  
   - Verified hamburger button visibility, drawer sliding transitions, backdrop blur activation, close button dismiss, and `Esc` keyboard handler.
3. **Suite 3: Code Card Header Architecture & Copy Button Anchoring** (6/6 checks)  
   - Verified `.code-card-header` fixed anchoring, prevented button floating over scrolled code, checked copy feedback transition (`✓ Copied!`), and verified toast alerts.
4. **Suite 4: 18-Theme Dynamic Palette Engine** (4/4 checks)  
   - Verified theme selection via navbar `<select>` and gallery cards, tested live CSS variable injections, and validated OLED `#000000` pitch-black tokens.
5. **Suite 5: Interactive Screenshot Showcase & Keyboard Navigation** (4/4 checks)  
   - Verified tab click switching, image source swap, caption synchronization, and numerical keyboard hotkeys (`1` to `6`).
6. **Suite 6: Keybindings Live Search Filtering** (1/1 check)  
   - Verified instant table row filtering upon query inputs.
7. **Suite 7: FAQ Hub Search & Category Filtering** (2/2 checks)  
   - Verified category pill filtering and keyword search matching on `faqs.html`.

---

## 6. Automated Source Code Sanity & Fact-Checking Audit Suite

**Test Script:** [`scripts/sanity_and_fact_check.mjs`](scripts/sanity_and_fact_check.mjs)  
**Execution Command:** `make check-facts` or `node scripts/sanity_and_fact_check.mjs`  
**Pass Rate:** **81 / 81 Checks Passed (100% Invariants Verified)**  

### Audit Category Breakdown

| Category | Assertions | Source Truth Invariants Audited | Status |
| :--- | :---: | :--- | :---: |
| **Rust Source Invariants** | 31 | All 18 `ThemeChoice` variants, 528Hz/1056Hz/1584Hz Work chime harmonics, 587.33Hz/880.0Hz Short Break tones, 523.25Hz/659.25Hz/783.99Hz Long Break triad, 25/5/15m config bounds | **PASS** (31/31) |
| **HTML Assets & Shell Structure** | 18 | Viewport meta, UTF-8 charset, favicon, mobile drawer & backdrop across `index.html`, `features.html`, and `faqs.html` | **PASS** (18/18) |
| **Screenshot Asset Integrity** | 6 | All 6 high-contrast KDE Konsole PNG assets exist on disk with valid file sizes (50 KB to 105 KB) | **PASS** (6/6) |
| **18-Theme CSS & JS Parity** | 18 | 1:1 synchronization between `src/theme.rs`, `docs/style.css` (`data-theme`), `docs/app.js`, and HTML selectors | **PASS** (18/18) |
| **Web Audio Synthesizer Math** | 3 | Exact mathematical frequency formulas match Rust `src/audio.rs` | **PASS** (3/3) |
| **Install Code Card Anchoring** | 5 | All 5 OS install panels use structured `.code-card` headers with anchored copy buttons | **PASS** (5/5) |
| **Total Fact-Check Assertions** | **81** | **All Invariants Match Production Codebase & Runtime Truth** | **100% PASS** |
