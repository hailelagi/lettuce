use std::cmp;

// a rust CATree port based on:
// * https://doi.org/10.1016/j.jpdc.2017.11.007
// * https://dl.acm.org/doi/10.1145/2633448.2633455

// reference implementation:
// https://github.com/erlang/otp/blob/629597747844b45a79458bde84cf1d7440aad46e/erts/emulator/beam/erl_db_catree.h
// https://github.com/erlang/otp/blob/629597747844b45a79458bde84cf1d7440aad46e/erts/emulator/beam/erl_db_catree.c

pub enum LockState {
    LOCK_FAILURE_CONTRIBUTION,
    LOCK_SUCCESS_CONTRIBUTION,
    LOCK_GRAVITY_CONTRIBUTION,
    LOCK_GRAVITY_PATTERN,
    LOCK_MORE_THAN_ONE_CONTRIBUTION,
    HIGH_CONTENTION_LIMIT,
    LOW_CONTENTION_LIMIT,
    MAX_ROUTE_NODE_LAYER_HEIGHT,
    LOCK_LOW_NO_CONTRIBUTION_LIMIT,
    LOCK_HIGH_NO_CONTRIBUTION_LIMIT,
}

impl<K: Ord, V> CATree<K, V> {
    pub fn new() -> Self {
        CATree { root: None }
    }
}

/*
// todo: port over some of these into a proper Cursor/Stream Api
    db_create_catree,
    db_first_catree,
    db_next_catree,
    db_last_catree,
    db_prev_catree,
    db_put_catree,
    db_get_catree,
    db_get_element_catree,
    db_member_catree,
    db_erase_catree,
    db_erase_object_catree,
    db_slot_catree,
    db_select_chunk_catree,
    db_select_catree,
    db_select_delete_catree,
    db_select_continue_catree,
    db_select_delete_continue_catree,
    db_select_count_catree,
    db_select_count_continue_catree,
    db_select_replace_catree,
    db_select_replace_continue_catree,
    db_take_catree,
    db_delete_all_objects_catree,
    db_delete_all_objects_get_nitems_from_holder_catree,
    db_free_table_catree,
    db_free_table_continue_catree,
    db_print_catree,
    db_foreach_offheap_catree,
    db_lookup_dbterm_catree,
    db_finalize_dbterm_catree,
    db_eterm_to_dbterm_tree_common,
    db_dbterm_list_append_tree_common,
    db_dbterm_list_remove_first_tree_common,
    db_put_dbterm_catree,
    db_free_dbterm_tree_common,
    db_get_dbterm_key_tree_common,
    db_get_binary_info_catree,
    db_first_catree, /* raw_first same as first */
    db_next_catree,   /* raw_next same as next */
    db_first_lookup_catree,
    db_next_lookup_catree,
    db_last_lookup_catree,
    db_prev_lookup_catree
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut tree = CATree::new();
        tree.set(1, "one");
        tree.set(2, "two");
        tree.set(3, "three");

        assert_eq!(tree.get(&1), Some(&"one"));
        assert_eq!(tree.get(&2), Some(&"two"));
        assert_eq!(tree.get(&3), Some(&"three"));
        assert_eq!(tree.get(&4), None);
    }
}
