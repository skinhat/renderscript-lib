// Based upon http://www.thegeekstuff.com/2012/08/c-linked-list-example/ linked list example
// Execute in Android with mScript.invoke_main();

##pragma version(1)
#pragma rs java_package_name(com.skinhat.segment)
#pragma stateFragment(parent)

#include "rs_debug.rsh"

char *memory;
static int memoryinc;

static char  *malloc(int bytes)
{
   char*ret=memory+	memoryinc;
	memoryinc+=bytes;
   return ret;
}


struct test_struct
{
    short *val;
    void *next;
};

static struct test_struct *head = NULL;
static struct test_struct *curr = NULL;

static int silent=1;

static struct test_struct* create_list( char *val,struct test_struct *ptr)
{

    if(NULL == ptr)
    {
	    if (!silent)
    	    rsDebug("\n Node creation failed \n",1);
        return NULL;
    }
    ptr->val = val;
    ptr->next = NULL;

    head = curr = ptr;
    return ptr;
}

static struct test_struct* add_to_list(char *val, bool add_to_end,struct test_struct *ptr)
{
    if(NULL == head) {
        
        return (create_list(val,ptr));
    }

    /*if (!silent){

	    if(add_to_end)
	        rsDebug("\n Adding node to end of list with value [%d]\n");//,val[0],val[1],val[2]);
	    else
	        rsDebug("\n Adding node to beginning of list with value [%d]\n");//,val[0],val[1],val[2]);
    }*/


    if(NULL == ptr) {
        rsDebug("\n Node creation failed \n",1);
        return NULL;
    }
    ptr->val = val;
    ptr->next = NULL;

    if(add_to_end) {
        curr->next = ptr;
        curr = ptr;
    }else {
        ptr->next = head;
        head = ptr;
    }
    return ptr;
}

static struct test_struct* add_first(char *val,struct test_struct *ptr)
{
    add_to_list(val,false,ptr);
}

static struct test_struct* add_last(char *val,struct test_struct *ptr)
{
    add_to_list(val,true,ptr);
}


static struct test_struct* search_in_list(char *val, struct test_struct **prev)
{
    struct test_struct *ptr = head;
    struct test_struct *tmp = NULL;
    bool found = false;

    //if (!silent)
    //	rsDebug("\n Searching the list for value [%d] \n",val[0],val[1],val[2]);

    while(ptr != NULL) {
        if(ptr->val == val){
            found = true;
            break;
        }  else   {
            tmp = ptr;
            ptr = ptr->next;
        }
    }

    if(true == found)  {
        if(prev)
            *prev = tmp;
        return ptr;
    } else {
        return NULL;
    }
}


static int delete_from_list(char* val)
{
    struct test_struct *prev = NULL;
    struct test_struct *del = NULL;

   // if (!silent)
	//    rsDebug("\n Deleting value [%d] from list\n",val[0],val[1],val[2]);

    del = search_in_list(val,&prev);
    if(del == NULL) {
        return -1;
    } else {

        if(prev != NULL) {
            prev->next = del->next;
        } 
        if(del == curr) {
            curr = prev;
            if (del==head)
                head=curr;
        } else if(del == head){
            head = del->next;
        } 
    }

   // free(del);
    del = NULL;

    return 0;
}

static char* remove_first()
{
    char* ret=head->val;
    delete_from_list(head->val);
    return ret;
}

static char* remove_last()
{
    char* ret=curr->val;
    delete_from_list(curr->val);
    return ret;
}

static bool is_empty()
{
   
    return head==NULL;
}

static int list_size()
{
  struct test_struct *ptr = head;
    int cnt=0;
    
    while(ptr != NULL)
    {
	  
        ptr = ptr->next;
        cnt++;
    }
    return cnt;
}

static void list_clear()
{
    memoryinc=0;
    head=NULL;
    curr=NULL;
}

static void print_list()
{
    struct test_struct *ptr = head;

    if (!silent)
	    rsDebug("\n -------Printing list Start------- \n",1);
    while(ptr != NULL)
    {
	    if (!silent) {
	        short *p=ptr->val;
    	    rsDebug("\n [%d] \n",p[0],p[1]);
    	}
        ptr = ptr->next;
    }
    if (!silent)
	    rsDebug("\n -------Printing list End------- \n",1);

    return;
}

void main()
{
    char memoryalloc[64000];
    memory=memoryalloc;
    memoryinc=0;
    
    
    int i = 0, ret = 0;
    struct test_struct *ptr = NULL;

    struct test_struct test_structs[100];
    int structinc=0;
    
    print_list();

    for(i = 5; i<10; i++) {
        short *p=malloc(2*sizeof(short));
        p[0]=i;
        p[1]=i*2;
        struct test_struct *element= malloc(sizeof(struct test_struct));
     
        add_first(p,element);
        
    }

    rsDebug("size",list_size());

    while (!is_empty()) {
        short *p=remove_first();
        rsDebug("p[0] p[1]",p[0],p[1]);
    }
    rsDebug("size after ",list_size());

    

    //return 0;
}
