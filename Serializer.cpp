class Codec {
public:

    // Encodes a tree to a single string.
    string serialize(TreeNode* root) {
        if(root==NULL){
            return "";
        }
        string s="";
        queue<TreeNode*>q;
        q.push(root);
        while(!q.empty()){
            TreeNode*temp=q.front();
            q.pop();
            if(temp==NULL){
                s+="#,";
            }
            else{
                s=s+to_string(temp->val);
                s+=',';
                
            }
            if(temp!=NULL){
                q.push(temp->left);
                q.push(temp->right);
            }
        }
        return s;
        
    }

    // Decodes your encoded data to tree.
    TreeNode* deserialize(string data) {
        if(data.size()==0){
            return NULL;
        }
        queue<TreeNode*>q;
        
        stringstream s(data);
        string str;
        getline(s, str, ',');
        TreeNode*temp= new TreeNode(stoi(str));
        q.push(temp);
        while(!q.empty()){
            TreeNode*root=q.front();
            q.pop();
            getline(s,str,',');
            if(str=="#"){
                root->left=NULL;
            }
            else{
                TreeNode*leftnode=new TreeNode(stoi(str));
                root->left=leftnode;
                q.push(leftnode);
            }
            getline(s,str,',');
            if(str=="#"){
                root->right=NULL;
            }
            else{
                TreeNode*rightnode=new TreeNode(stoi(str));
                root->right=rightnode;
                q.push(rightnode);
            }

        }
        return temp;
        
    }
};
