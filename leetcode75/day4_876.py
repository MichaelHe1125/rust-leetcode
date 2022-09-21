from typing import List, Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val: int = val
        self.next: Optional[ListNode] = next


def create_link(nums: List[int]) -> Optional[ListNode]:
    """创建链表"""
    if len(nums) == 0:
        return None
    head = ListNode(nums[0])
    cur = head
    for num in nums[1:]:
        node = ListNode(num)
        cur.next = node
        cur = node
    return head


def trave_link(head: Optional[ListNode]) -> List[int]:
    """遍历链表"""
    nums = []
    if head is None:
        return nums
    while head:
        nums.append(head.val)
        head = head.next
    return nums


class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(-1)
        while head:
            cur = head
            head = head.next
            cur.next = dummy.next
            dummy.next = cur
        return dummy.next


if __name__ == "__main__":
    head1 = create_link([1, 2, 4])
    head = Solution().reverseList(head1)
    assert trave_link(head) == [4, 2, 1]

    head1 = create_link([1])
    head = Solution().reverseList(head1)
    assert trave_link(head) == [1]

    head1 = create_link([])
    head = Solution().reverseList(head1)
    assert trave_link(head) == []