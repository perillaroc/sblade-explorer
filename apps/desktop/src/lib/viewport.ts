import { ref } from "vue";

/**
 * Reactive viewport state for the F0 narrow-window rules.
 * Below 1100px the sidebar auto-collapses to the w-14 icon rail and the
 * topbar hides secondary text (see 03-ia-wireframes.md §7).
 */
const NARROW_QUERY = "(max-width: 1099px)";

const media = window.matchMedia(NARROW_QUERY);

export const isNarrow = ref(media.matches);

media.addEventListener("change", (event) => {
  isNarrow.value = event.matches;
});
