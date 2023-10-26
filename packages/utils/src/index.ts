export const add = (a: number, b: number) => {
  return a + b;
};

export const subtract = (a: number, b: number) => {
  return a - b;
};

export type { BigFlow } from "./supabase/fetchSupabase";
export {
  fetchProfile,
  fetchProfiles,
  fetchProfileTemplates,
  fetchTemplateBySlug,
  fetchTemplates,
} from "./supabase/fetchSupabase";
export { supabaseClient } from "./supabase/client";
export {
  flowJsonFromBigFlow,
  getAProfileLink,
  formatUrl,
  hasLinks,
} from "./supabase/helpers";
export type {
  Database,
  Json,
  Profile,
  Tag,
} from "./supabase/types/supabase.types";
export * from "./types/flow";
