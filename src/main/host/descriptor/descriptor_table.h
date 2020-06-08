/*
 * The Shadow Simulator
 * See LICENSE for licensing information
 */

#ifndef SRC_MAIN_HOST_DESCRIPTOR_DESCRIPTOR_TABLE_H_
#define SRC_MAIN_HOST_DESCRIPTOR_DESCRIPTOR_TABLE_H_

#include <stdbool.h>

#include "main/host/descriptor/descriptor_types.h"

/* Opaque object to store the state needed to implement the module. */
typedef struct _DescriptorTable DescriptorTable;

/* Create an object that can be used to store all descriptors created by
 * a process. The reference count starts at 1; when the table is no longer
 * required, use unref() to release the reference.*/
DescriptorTable* descriptortable_new();

/* Increment the reference count for this table. */
void descriptortable_ref(DescriptorTable* table);

/* Decrement the reference count and free the table if no refs remain. */
void descriptortable_unref(DescriptorTable* table);

/* Store a descriptor object for later reference at the next available index
 * in the table. The chosen table index is stored in the descriptor object and
 * returned. The descriptor is guaranteed to be stored successfully.*/
int descriptortable_add(DescriptorTable* table, Descriptor* descriptor);

/* Stop storing the descriptor so that it can no longer be referenced. The table
 * index that was used to store the descriptor is cleared from the descriptor
 * and may be assigned to new descriptors that are later added to the table.
 * Returns true if the descriptor was found in the table and removed, and false
 * otherwise. */
bool descriptortable_remove(DescriptorTable* table, Descriptor* descriptor);

/* Returns the descriptor at the given table index, or NULL if we are not
 * storing a descriptor at the given index. */
Descriptor* descriptortable_get(DescriptorTable* table, int index);

/* Store the given descriptor at the special index reserved for STDOUT. Any
 * previous descriptor that was stored there will be removed and its table
 * index will be cleared. */
void descriptortable_setStdOut(DescriptorTable* table, Descriptor* descriptor);

/* Store the given descriptor at the special index reserved for STDERR. Any
 * previous descriptor that was stored there will be removed and its table
 * index will be cleared. */
void descriptortable_setStdErr(DescriptorTable* table, Descriptor* descriptor);

/* This is a helper function that handles some corner cases where some
 * descriptors are linked to each other and we must remove that link in
 * order to ensure that the reference count reaches zero and they are properly
 * freed. Otherwise the circular reference will prevent the free operation.
 * TODO: remove this once the TCP layer is better designed. */
void descriptortable_shutdownHelper(DescriptorTable* table);

#endif /* SRC_MAIN_HOST_DESCRIPTOR_DESCRIPTOR_TABLE_H_ */
