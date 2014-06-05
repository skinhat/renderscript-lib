#pragma version(1)
#pragma rs java_package_name(com.skinhat) 
#pragma stateFragment(parent)

#include "rs_debug.rsh"
struct test_struct
{
    int val;
    void *next;
};

static struct test_struct *head = NULL;
static struct test_struct *curr = NULL;


static struct test_struct* create_list( int val,struct test_struct *ptr)
{
    rsDebug("\n creating list with headnode as [%d]\n",val);
    //struct test_struct *ptr = (struct test_struct*)malloc(sizeof(struct test_struct));
    if(NULL == ptr)
    {
        rsDebug("\n Node creation failed \n",1);
        return NULL;
    }
    ptr->val = val;
    ptr->next = NULL;

    head = curr = ptr;
    return ptr;
}

static struct test_struct* add_to_list(int val, bool add_to_end,struct test_struct *ptr)
{
    if(NULL == head) {
        
        return (create_list(val,ptr));
    }

    if(add_to_end)
        rsDebug("\n Adding node to end of list with value [%d]\n",val);
    else
        rsDebug("\n Adding node to beginning of list with value [%d]\n",val);

   // struct test_struct *ptr = (struct test_struct*)malloc(sizeof(struct test_struct));
    if(NULL == ptr)
    {
        rsDebug("\n Node creation failed \n",1);
        return NULL;
    }
    ptr->val = val;
    ptr->next = NULL;

    if(add_to_end)
    {
        curr->next = ptr;
        curr = ptr;
    }
    else
    {
        ptr->next = head;
        head = ptr;
    }
    return ptr;
}

static struct test_struct* search_in_list( int val, struct test_struct **prev)
{
    struct test_struct *ptr = head;
    struct test_struct *tmp = NULL;
    bool found = false;

    rsDebug("\n Searching the list for value [%d] \n",val);

    while(ptr != NULL)
    {
        if(ptr->val == val)
        {
            found = true;
            break;
        }
        else
        {
            tmp = ptr;
            ptr = ptr->next;
        }
    }

    if(true == found)
    {
        if(prev)
            *prev = tmp;
        return ptr;
    }
    else
    {
        return NULL;
    }
}

static int delete_from_list(int val)
{
    struct test_struct *prev = NULL;
    struct test_struct *del = NULL;

    rsDebug("\n Deleting value [%d] from list\n",val);

    del = search_in_list(val,&prev);
    if(del == NULL)
    {
        return -1;
    }
    else
    {
        if(prev != NULL)
            prev->next = del->next;

        if(del == curr)
        {
            curr = prev;
        }
        else if(del == head)
        {
            head = del->next;
        }
    }

   // free(del);
    del = NULL;

    return 0;
}

static void print_list()
{
    struct test_struct *ptr = head;

    rsDebug("\n -------Printing list Start------- \n",1);
    while(ptr != NULL)
    {
        rsDebug("\n [%d] \n",ptr->val);
        ptr = ptr->next;
    }
    rsDebug("\n -------Printing list End------- \n",1);

    return;
}

void main()
{

	struct test_struct *head = NULL;
	struct test_struct *curr = NULL;

    int i = 0, ret = 0;
    struct test_struct *ptr = NULL;

    struct test_struct test_structs[100];
    int structinc=0;
    
    print_list();

    for(i = 5; i<10; i++) {
        add_to_list(i,true,&test_structs[structinc++]);
    }

    print_list();

    for(i = 4; i>0; i--) {
      
        add_to_list(i,false,&test_structs[structinc++]);
    }

    print_list(head);

    for(i = 1; i<10; i += 4)
    {
        ptr = search_in_list(i, NULL);
        if(NULL == ptr)
        {
            rsDebug("\n Search [val = %d] failed, no such element found\n",i);
        }
        else
        {
            rsDebug("\n Search passed [val = %d]\n",ptr->val);
        }

        print_list();

        ret = delete_from_list(i);
        if(ret != 0)
        {
            rsDebug("\n delete [val = %d] failed, no such element found\n",i);
        }
        else
        {
            rsDebug("\n delete [val = %d]  passed \n",i);
        }

        print_list();
    }

    //return 0;
}
